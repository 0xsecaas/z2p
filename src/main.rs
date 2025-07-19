use std::net::TcpListener;
mod startup;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("Listening to {}", listener.local_addr().unwrap().port());
    startup::run(listener)?.await
}
