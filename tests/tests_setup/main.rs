use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[path = "./test_app.rs"]
mod test_app;
use test_app::{TestApp, DB_NAME_PREFIX};

const BASE_URL: &str = "127.0.0.1";

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();

    let test_log = std::env::var("TEST_LOG").is_ok();

    if test_log {
        let subscriber = get_subscriber(
            "zero2prod-test".into(),
            default_filter_level,
            std::io::stdout,
        );
        init_subscriber(subscriber);
    } else {
        let subscriber =
            get_subscriber("zero2prod-test".into(), default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

/**
 * We need to refactor our project into a library and a binary: all our logic will live in the library crate
while the binary itself will be just an entrypoint with a very slim main function
 */

pub async fn init(url: &str) -> TestApp {
    let mut app = spawn_app().await;
    app.address = format!("{}{}", app.address, url);
    return app;
}

// Launch our application in the background ~somehow~
async fn spawn_app() -> TestApp {
    // We take the BASE_URL const and assign it a port 0. We then
    // pass the listener to the server
    let base_url = format!("{}:0", BASE_URL);
    let listener = TcpListener::bind(base_url).expect("Failed to bind random port");

    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution
    Lazy::force(&TRACING);

    // We retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();

    let (connection, db_connection, db_name, connection_string) = init_db().await;

    // We pass the port now to our server
    let server = zero2prod::run(listener, db_connection.clone()).expect("Failed to bind address");
    let _ = actix_web::rt::spawn(server);
    let address = format!("http://{}:{}", BASE_URL, port);

    TestApp {
        address,
        db_name,
        connection_string,
        db_pool: db_connection,
        connection,
    }
}

async fn init_db() -> (PgConnection, PgPool, String, String) {
    let mut configuration = get_configuration().expect("Failed to read configuration");

    // We change the db name in each run as we need to run the test multiple times
    let db_name = format!("{}{}", DB_NAME_PREFIX, Uuid::new_v4().to_string());

    configuration.database.database_name = db_name;

    let (connection, pool) = configure_database(&configuration.database).await;

    return (
        connection,
        pool,
        configuration.database.database_name.clone(),
        configuration.database.connection_string_without_db(),
    );
}

async fn configure_database(config: &DatabaseSettings) -> (PgConnection, PgPool) {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}""#, config.database_name).as_str())
        .await
        .expect("Failed to create the db");

    // Migrate the database

    let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate db");

    return (connection, connection_pool);
}
