use sqlx::{Connection, Executor, PgConnection, PgPool};

pub const DB_NAME_PREFIX: &str = "test_db-";

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub db_name: String,
    pub connection_string: String,
    pub connection: PgConnection,
}

impl TestApp {
    async fn terminate(&mut self) {
        assert!(self.db_name.starts_with("test_db-"));
        println!("Cleaning up database: {}", self.db_name);

        self.db_pool.close().await;

        let mut connection = PgConnection::connect(&self.connection_string)
            .await
            .expect("Error while obtaining the connection");
        // .expect("Failed to connect to Postgres");

        // Force drop all active connections to database
        // TODO: see if there is a softer way to handle this (i.e. close connection when DB access is complete)
        connection
            .execute(
                format!(
                    r#"
                    SELECT pg_terminate_backend(pg_stat_activity.pid)
                    FROM pg_stat_activity
                    WHERE pg_stat_activity.datname = '{}'
                    AND pid <> pg_backend_pid()
                    "#,
                    self.db_name
                )
                .as_str(),
            )
            .await
            .expect("Error while executing the PG Connection");
        // .expect("Failed to terminate current connections to test db");

        connection
            .execute(format!(r#"DROP DATABASE "{}";"#, self.db_name).as_str())
            .await
            .expect("DROP the database error the connection");
        // .expect("Failed to drop database.");
        println!("Database cleaned up successfully.");
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        std::thread::scope(|s| {
            s.spawn(|| {
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                runtime.block_on(self.terminate());
            });
        });
    }
}
