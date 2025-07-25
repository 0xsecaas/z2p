use sqlx::postgres::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configurationg.");
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("Listening to {}", listener.local_addr().unwrap().port());
    run(listener)?.await?;
    Ok(())
}
