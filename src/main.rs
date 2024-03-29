use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

use dotenv;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

const BASE_URL: &str = "127.0.0.1";

/**
Everything was moved to lib.rs to prevent clashes
*/
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // For adding logging functionality
    // dotenv was added to load the RUST_LOG env variable from .env
    // This will load it into env_logger which actix will use to generate
    // logs.
    // This is what we say when we talk about a local decision.
    dotenv::dotenv().ok();

    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let (listener, port) = get_listener();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    println!("Listening on {}:{}", BASE_URL, port);

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
