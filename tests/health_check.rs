// use zero2prod::main;

use rstest::rstest;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // arrange
    let host = spawn_app();
    let client = reqwest::Client::new();

    // act
    let response = client
        .get(format!("{}/healthcheck", host))
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn given_valid_request_when_subscribe_then_200() {
    // arrange
    let host = spawn_app();
    let client = reqwest::Client::new();

    // act
    let response = client
        .post(format!("{}/subscriptions", host))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("name=dummy&email=test%40test.com")
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!(Some(0), response.content_length());
}

#[rstest]
#[case("email=test%40test.com")]
#[case("name=dummy")]
#[case("")]
#[tokio::test]
async fn given_invalid_body_when_subscribe_then_400(#[case] invalid_body: String) {
    // arrange
    let host = spawn_app();
    let client = reqwest::Client::new();

    // act
    let response = client
        .post(format!("{}/subscriptions", host))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(400, response.status().as_u16());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to spawn our app");
    tokio::spawn(server);

    format!("http://localhost:{}", port)
}
