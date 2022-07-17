use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};

const BASE_URL: &str = "127.0.0.1";

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

/**
 * We need to refactor our project into a library and a binary: all our logic will live in the library crate
while the binary itself will be just an entrypoint with a very slim main function
 */

pub async fn init(url: &str) -> (String, PgPool) {
    let app = spawn_app().await;
    let addr = format!("{}{}", app.address, url);
    return (addr, app.db_pool);
}

// Launch our application in the background ~somehow~
async fn spawn_app() -> TestApp {
    // We take the BASE_URL const and assign it a port 0. We then
    // pass the listener to the server
    let base_url = format!("{}:0", BASE_URL);
    let listener = TcpListener::bind(base_url).expect("Failed to bind random port");

    // We retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();

    let db_connection = init_db().await;

    // We pass the port now to our server
    let server = zero2prod::run(listener, db_connection.clone()).expect("Failed to bind address");
    let _ = actix_web::rt::spawn(server);
    let address = format!("http://{}:{}", BASE_URL, port);

    TestApp {
        address,
        db_pool: db_connection,
    }
}

async fn init_db() -> PgPool {
    let mut configuration = get_configuration().expect("Failed to read configuration");

    // We change the db name in each run as we need to run the test multiple times
    configuration.database.database_name = Uuid::new_v4().to_string();

    return configure_database(&configuration.database).await;
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}""#, config.database_name).as_str())
        .await
        .expect("Failed to create the db");

    // Migrate the database

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate db");

    return connection_pool;
}
