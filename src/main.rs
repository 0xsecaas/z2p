use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("WILLKOMMEN!\n");
    let subscriber = get_subscriber("z2p".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configurationg.");
    let connection = PgPool::connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect to Postgres");

    let listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))?;
    println!("Listening to {}", listener.local_addr().unwrap().port());
    run(listener, connection)?.await?;
    Ok(())
}
