use crate::helpers::{assert_is_redirected_to, spawn_app};

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password",
    });

    let response = app.post_login(&login_body).await;

    let flash_cookies = response
        .cookies()
        .find(|cookie| cookie.name() == "_flash")
        .unwrap();

    // Assert
    assert_is_redirected_to(&response, "/login");
    assert_eq!(flash_cookies.value(), "Authentication failed");
}
