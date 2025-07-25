use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration::{self, get_configuration};
use zero2prod::startup::run;

// You can inspect what code gets generated using
// `cargo expand --test health_check`
pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let port = listener.local_addr().unwrap().port();
    let server = run(listener, connection).expect("Failed to bind address");

    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
