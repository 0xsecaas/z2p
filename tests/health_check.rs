use std::net::TcpListener;

// You can inspect what code gets generated using
// `cargo expand --test health_check`

#[actix_rt::test]
// is the testing equivalent of the actix_web::main
// no need to add #[test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();

    //let client = reqwest::Client::new();
    let client = reqwest::Client::builder()
        .no_proxy()
        .build()
        .expect("Failed to build client");
    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");

    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
