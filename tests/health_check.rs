mod common;
use common::spawn_app;

#[actix_rt::test]
// is the testing equivalent of the actix_web::main
// no need to add #[test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;

    //let client = reqwest::Client::new();
    let client = reqwest::Client::builder()
        .no_proxy()
        .build()
        .expect("Failed to build client");
    // Act
    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
