use crate::helpers::{spawn_app, VirtualDB};

#[tokio::test]
async fn health_check_works() {
    // Arrange ->
    let app = spawn_app(VirtualDB::Disabled).await;

    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act ->
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert ->
    println!("{:?}", response.content_length());
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}
