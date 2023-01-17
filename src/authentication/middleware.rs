use std::ops::Deref;

use crate::{
    session_state::TypedSession,
    utils::{error500, see_other},
};
use actix_web::error::InternalError;
use actix_web::{body::MessageBody, FromRequest};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    HttpMessage,
};
use actix_web_lab::middleware::Next;
use uuid::Uuid;

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        TypedSession::from_request(http_request, payload).await?
    };

    match session.get_user_id().map_err(error500)? {
        Some(user_id) => {
            req.extensions_mut().insert(UserId(user_id));
            next.call(req).await
        }
        None => {
            let response = see_other("/login");
            let error = anyhow::anyhow!("the user has not logged in");
            Err(InternalError::from_response(error, response).into())
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct UserId(Uuid);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for UserId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
