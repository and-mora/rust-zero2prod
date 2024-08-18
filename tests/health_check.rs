use rstest::rstest;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::run;

#[tokio::test]
async fn health_check_works() {
    // arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // act
    let response = client
        .get(format!("{}/healthcheck", test_app.host))
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
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // act
    let response = client
        .post(format!("{}/subscriptions", test_app.host))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("name=dummy&email=test%40test.com")
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!(Some(0), response.content_length());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&test_app.pool)
        .await
        .expect("failed to save sub");
    assert_eq!(saved.email, "test@test.com");
    assert_eq!(saved.name, "dummy");
}

#[rstest]
#[case("email=test%40test.com")]
#[case("name=dummy")]
#[case("")]
#[tokio::test]
async fn given_invalid_body_when_subscribe_then_400(#[case] invalid_body: String) {
    // arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // act
    let response = client
        .post(format!("{}/subscriptions", test_app.host))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(400, response.status().as_u16());
}

async fn spawn_app() -> TestApp {
    let mut configuration = get_configuration().unwrap();
    configuration.database.database_name = String::from(Uuid::new_v4());
    let connection_string = configuration.database.connection_string();
    let pool = configure_database(&configuration.database);

    let listener = TcpListener::bind("localhost:0").expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, pool.clone()).expect("Failed to spawn our app");
    tokio::spawn(server);

    TestApp {
        pool,
        host: format!("http://localhost:{}", port),
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("failed to connect db.");

    // create the brand-new database
    connection
        .execute(format!("CREATE DATABASE {}", config.database_name).as_str())
        .await
        .expect("failed to create db");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("failed to connect");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("failed to migrate db");

    connection_pool
}

pub struct TestApp {
    pool: PgPool,
    host: String,
}
