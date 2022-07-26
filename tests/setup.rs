use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use std::thread;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};

const BASE_URL: &str = "127.0.0.1";

pub struct TestApp {
    db_name: String,
    connection_string: String,
    pub address: String,
    pub db_pool: PgPool,
    pub connection: PgConnection,
}

impl TestApp {
    async fn terminate(&mut self) {
        self.connection
            .execute(format!(r#"DROP DATABASE "{}""#, &self.db_name).as_str())
            .await
            .expect("Failed to create the db");
    }
}

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

    // We retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();

    let (connection, db_connection, db_name, connection_string) = init_db().await;

    // We pass the port now to our server
    let server = zero2prod::run(listener, db_connection.clone()).expect("Failed to bind address");
    let _ = actix_web::rt::spawn(server);
    let address = format!("http://{}:{}", BASE_URL, port);

    TestApp {
        db_name: String::from(db_name),
        address,
        db_pool: db_connection,
        connection,
        connection_string,
    }
}

async fn init_db() -> (PgConnection, PgPool, String, String) {
    let mut configuration = get_configuration().expect("Failed to read configuration");

    // We change the db name in each run as we need to run the test multiple times
    configuration.database.database_name = Uuid::new_v4().to_string();

    let (connection, pool) = configure_database(&configuration.database).await;

    return (
        connection,
        pool,
        String::from(&configuration.database.database_name),
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

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate db");

    return (connection, connection_pool);
}

// impl Drop for TestApp {
//     fn drop(&mut self) {
//         thread::scope(|s| {
//             s.spawn(|| {
//                 let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
//                 runtime.block_on(self.terminate());
//             });
//         });
//     }
// }
