const BASE_URL: &str = "http://127.0.0.1:8000";

/**
 * We need to refactor our project into a library and a binary: all our logic will live in the library crate
while the binary itself will be just an entrypoint with a very slim main function
 */

#[actix_web::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();
    let url = format!("{}/health_check", BASE_URL);
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

// Launch our application in the background ~somehow~
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = actix_web::rt::spawn(server);
}
