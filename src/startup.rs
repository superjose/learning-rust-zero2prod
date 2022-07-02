use crate::routes::{create_subscription, health_check};
use actix_web::web::Data;

use actix_web::{dev::Server, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    let connection = Data::new(connection);

    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(create_subscription)
            // Get a pointer and attach it to the application state
            .app_data(connection.clone())
        // .route("/hey", web::get().to(manual_hello))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
