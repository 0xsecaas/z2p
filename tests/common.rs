use std::net::TcpListener;
use zero2prod::startup::run;

// You can inspect what code gets generated using
// `cargo expand --test health_check`
pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");

    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
