use zero2prod::run;

/**
 * Everything was moved to lib.rs to prevent clashes
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run()?.await
}
