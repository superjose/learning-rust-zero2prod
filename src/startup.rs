use crate::routes::{health_check, subscribe};
use actix_web::web::Data;

use actix_web::{dev::Server, middleware::Logger, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(health_check)
            .service(subscribe)
            // Get a pointer and attach it to the application state
            .app_data(connection.clone())
        // .route("/hey", web::get().to(manual_hello))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
