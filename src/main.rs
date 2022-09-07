use sqlx::PgPool;
use std::net::TcpListener;

use dotenv;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

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
    // `init` call `set_logger`, so this is all we need to do.
    //  We are also falling back to printing all the logs at info-level
    //  or above if the RUST_LOG environment variable has not been set.

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        //Output the formatted spans to stdout
        std::io::stdout,
    );

    // The `with` method is provided by `SubscriberExt`, an extension
    // trait for `Subscriber` exposed by `tracing_subscriber`
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");

    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let (listener, port) = get_listener();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgPool::connect(&configuration.database.connection_string())
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
