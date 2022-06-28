use std::net::TcpListener;

use actix_web;
use rstest::rstest;
use sqlx::{Connection, PgConnection};
use urlencoding::encode;
use zero2prod::configuration::get_configuration;

const BASE_URL: &str = "127.0.0.1";

/**
 * We need to refactor our project into a library and a binary: all our logic will live in the library crate
while the binary itself will be just an entrypoint with a very slim main function
 */

#[actix_web::test]
async fn health_check_works() {
    // Arrange
    let client = reqwest::Client::new();
    let url = init("/health_check");

    // Act
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn init(url: &str) -> String {
    let base_url_with_port = spawn_app();
    format!("{}{}", base_url_with_port, url)
}

// Launch our application in the background ~somehow~
fn spawn_app() -> String {
    // We take the BASE_URL const and assign it a port 0. We then
    // pas the listener to the server
    let base_url = format!("{}:0", BASE_URL);
    let listener = TcpListener::bind(base_url).expect("Failed to bind random port");

    // We retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();

    // We pass the port now to our server
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = actix_web::rt::spawn(server);
    format!("http://{}:{}", BASE_URL, port)
}

async fn init_db() -> PgConnection {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    // The 'Connection' trait must be in scope for us to invoke
    // 'PgConnection::connect - it is not an inherent method of the struct!
    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    return connection;
}

#[actix_web::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Arrange
    let app_address = init("/subscriptions");
    let connection = init_db().await;
    let client = reqwest::Client::new();
    let encName = encode("Night Stucker");
    let encEmail = encode("superjose_49@hotmail.com");
    let body = format!("name={}&email={}", encName, encEmail);

    // Act
    let response = client
        .post(app_address)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "Night Stucker");
    assert_eq!(saved.name, "superjose_49@hotmail.com");
}

#[actix_web::test]
async fn subscribe_returns_400_when_data_is_missing() {
    // Arrange
    let app_address = init("/subscriptions");
    let client = reqwest::Client::new();
    let name = encode("Night Stucker");
    let email = encode("superjose_49@hotmail.com");

    let enc_name = format!("name={}", name);
    let enc_email = format!("email={}", email);

    // Table driven tests.
    //Each of the following is a test case!
    let test_cases = vec![
        (enc_name, "missing email"),
        (enc_email, "missing name"),
        (String::from(""), "missing both email and name"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&app_address)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

// Using a parametrized version of the function above:
#[rstest]
#[case("Night Stucker", "", "missing email")]
#[actix_web::test]
async fn parametrized_subscribe_returns_400_when_data_is_missing(
    #[case] name: &str,
    #[case] email: &str,
    #[case] error_message: &str,
) {
    let app_address = init("/subscriptions");
    let client = reqwest::Client::new();

    let invalid_body = format!("name={}email={}", encode(name), encode(email));
    // Act
    let response = client
        .post(&app_address)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(
        400,
        response.status().as_u16(),
        "The API did not fail with 400 Bad Request when the payload was {}.",
        error_message
    );
}
