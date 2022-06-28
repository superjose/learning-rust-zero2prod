use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::run;

const BASE_URL: &str = "127.0.0.1";

/**
 * Everything was moved to lib.rs to prevent clashes
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = get_listener();
    let port = listener.local_addr().unwrap().port();

    print!("Listening on {}:{}", BASE_URL, port);

    run(listener)?
        .await
        .unwrap_or_else(|err| println!("{:?}", err));

    Ok(())
}

fn get_listener() -> TcpListener {
    let base_url = format!("{}:8080", BASE_URL);
    print!("{}", base_url);
    return TcpListener::bind(base_url).expect("Failed to bind random part");
}
