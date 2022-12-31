use actix_web::{web, HttpResponse, ResponseError};
use chrono::Utc;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::{domain::NewSubscriber, email_client::EmailClient, startup::ApplicationBaseUrl};

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[derive(serde::Serialize)]
struct BadRequestError<'a> {
    pub error: &'a str,
    pub message: String,
}

/* [subscribe] orchestrates the work to be done by calling the
 * required routines and translates their outcome into the
 * proper response according to the rules and conventions of
 * the HTTP protocol.
*/
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool, email_client, base_url),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url: web::Data<ApplicationBaseUrl>,
) -> Result<HttpResponse, actix_web::Error> {
    // `web::Form` is a wrapper around `FormData`
    // `form.0` gives us access to the underlying `FormData`
    // -->
    // We automatically get try_into for free by implementing
    // TryFrom<FormData> trait for NewSubscriber struct.
    let new_subscriber = match form.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    };

    // Transactions are a way to group together related
    // operations in a single unit of work.
    // The database guarantees that all operations within
    // a transaction will succeed or fail together
    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    // Insert subscriber into the DB and return subscriber ID ->
    let subscriber_id = match insert_subscriber(&mut transaction, &new_subscriber).await {
        Ok(subscriber_id) => subscriber_id,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let subscription_token = generate_subscription_token();

    // Insert subscriber_id and subscription_token into the DB ->
    // The `?` operator transparently invokes the `Into` trait
    // on our behalf - we don't need an explicit `map_err` anymore.
    store_token(&mut transaction, subscriber_id, &subscription_token).await?;

    // Commit the transaction to database ->
    if transaction.commit().await.is_err() {
        return Ok(HttpResponse::InternalServerError().finish());
    };

    // Send confirmation email along with the subscription link
    if send_confirmation_email(
        &email_client,
        new_subscriber,
        &base_url.0,
        &subscription_token,
    )
    .await
    .is_err()
    {
        return Ok(HttpResponse::InternalServerError().finish());
    }

    Ok(HttpResponse::Ok().finish())
}

/// Generate a random 25-characters-long case-sensitive subscription token.
fn generate_subscription_token() -> String {
    let mut rng = thread_rng();

    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}

#[tracing::instrument(
    name = "Send a confirmation email to a new subscriber",
    skip(email_client, new_subscriber, base_url)
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    new_subscriber: NewSubscriber,
    base_url: &str,
    subscription_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!(
        "{}/subscriptions/confirm?subscription_token={}",
        base_url, subscription_token
    );
    let plain_body = &format!(
        "Welcome to our newsletter!\nVisit {} to confirm your subscription.",
        confirmation_link
    );
    let html_body = &format!(
        "Welcome to our newsletter<br />\
        Click <a href=\"{}\">here</a> to confirm your subscription.",
        confirmation_link
    );

    email_client
        .send_email(new_subscriber.email, "Welcome!", html_body, plain_body)
        .await
}

/* Datebase Logic ->
 * [insert_subscriber] takes care of the database logic and it has
 * no awareness of the surrounding web framework - i.e. we are not
 * passing web::Form or web::Data wrappers as input types
 */
#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, transaction)
)]
pub async fn insert_subscriber(
    transaction: &mut Transaction<'_, Postgres>,
    new_subscriber: &NewSubscriber,
) -> Result<Uuid, sqlx::Error> {
    let subscriber_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'pending_confirmation')
        "#,
        subscriber_id,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now(),
    )
    .execute(transaction)
    .await
    .map_err(|err| {
        tracing::error!("Failed to execute query: {:?}", err);
        err
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;

    Ok(subscriber_id)
}

#[tracing::instrument(
    name = "Store subscription token in the database",
    skip(subscription_token, transaction)
)]
pub async fn store_token(
    transaction: &mut Transaction<'_, Postgres>,
    subscriber_id: Uuid,
    subscription_token: &str,
) -> Result<(), StoreTokenError> {
    sqlx::query!(
        r#"INSERT INTO subscription_tokens (subscription_token, subscriber_id)
        VALUES ($1, $2)"#,
        subscription_token,
        subscriber_id
    )
    .execute(transaction)
    .await
    .map_err(|error| {
        tracing::error!("Failed to execute query: {:?}", error);
        StoreTokenError(error)
    })?;

    Ok(())
}

// A new error type, wrapping a sqlx::Error
pub struct StoreTokenError(sqlx::Error);

impl ResponseError for StoreTokenError {}

// Implement Display Trait for StoreTokenError
impl std::fmt::Display for StoreTokenError {
    // Implement display ->
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while \
            trying to store a suscription token."
        )
    }
}

impl std::fmt::Debug for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl std::error::Error for StoreTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // The compiler transparently casts `&sqlx::Error` into a `&dyn Error`
        Some(&self.0)
    }
}

fn error_chain_fmt(
    error: &impl std::error::Error,
    formatter: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(formatter, "{}", error)?;

    let mut current = error.source();

    while let Some(cause) = current {
        writeln!(formatter, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }

    Ok(())
}
