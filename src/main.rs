use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

const BASE_URL: &str = "127.0.0.1";

/**
 * Everything was moved to lib.rs to prevent clashes
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let (listener, port) = get_listener();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    print!("Listening on {}:{}", BASE_URL, port);

    run(listener, connection)?
        .await
        .unwrap_or_else(|err| println!("{:?}", err));

    Ok(())
}

fn get_listener() -> (TcpListener, u16) {
    let configuration = get_configuration().expect("Failed to read configuration");
    let port = configuration.application_port;
    let base_url = format!("{}:{}", BASE_URL, port);
    print!("{}", base_url);
    let listener = TcpListener::bind(base_url).expect("Failed to bind random part");
    return (listener, port);
}
