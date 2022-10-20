//! tests/health_check.rs
use zero2prod::run;

#[tokio::test]
async fn dummy_test() {
    // Arrange
    spawn_app().await.expect("Failed to spawn app.");
}

async fn spawn_app() -> std::io::Result<()> {
    todo!()
}