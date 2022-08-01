use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;

use dotenv;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

const BASE_URL: &str = "127.0.0.1";

/**
Everything was moved to lib.rs to prevent clashes
*/
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // `init` call `set_logger`, so this is all we need to do.
    //  We are also falling back to printing all the logs at info-level
    //  or above if the RUST_LOG environment variable has not been set.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

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
