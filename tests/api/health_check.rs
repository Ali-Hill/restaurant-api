use crate::client::spawn_app;

#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;

    // Arrange
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
