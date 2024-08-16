// use zero2prod::main;

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

fn spawn_app() -> String {
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to spawn our app");
    tokio::spawn(server);

    format!("http://localhost:{}", port)
}