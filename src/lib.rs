//! src/lib.rs
pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::{dev::Server, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(routes::health_check)
            .service(routes::create_subscription)
        // .route("/hey", web::get().to(manual_hello))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
