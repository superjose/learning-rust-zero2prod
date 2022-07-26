use actix_web;
use rstest::rstest;
mod setup;

use urlencoding::encode;

#[actix_web::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Arrange
    let app = setup::init("/subscriptions").await;

    let client = reqwest::Client::new();
    let enc_name = encode("Night Stucker");
    let enc_email = encode("superjose_49@hotmail.com");
    let body = format!("name={}&email={}", enc_name, enc_email);

    // Act
    let response = client
        .post(&app.address)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to execute db");

    assert_eq!(saved.name, "Night Stucker");
    assert_eq!(saved.email, "superjose_49@hotmail.com");
}

#[actix_web::test]
async fn subscribe_returns_400_when_data_is_missing() {
    // Arrange
    let app = setup::init("/subscriptions").await;
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
            .post(&app.address)
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
    let app = setup::init("/subscriptions").await;
    let client = reqwest::Client::new();

    let invalid_body = format!("name={}email={}", encode(name), encode(email));
    // Act
    let response = client
        .post(&app.address)
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

#[actix_web::test]
async fn health_check_works() {
    // Arrange
    let client = reqwest::Client::new();
    let app = setup::init("/health_check").await;

    // Act
    let response = client
        .get(&app.address)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
