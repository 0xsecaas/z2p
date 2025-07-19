use crate::routes::{health_check, subscriptions};
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check::health_check)
            .route("/subscriptions", web::post().to(subscriptions::subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
