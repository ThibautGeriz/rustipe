extern crate recipes_backend;

use self::recipes_backend::infrastructure::web::server;

use rocket::http::{ContentType, Status};
use rocket::local::Client;

use dotenv::dotenv;
use std::env;

fn get_database_url() -> String {
    String::from(
        env::var("TEST_DATABASE_URL")
            .or_else(|_e| {
                dotenv().ok();
                env::var("TEST_DATABASE_URL")
            })
            .expect("TEST_DATABASE_URL must be set"),
    )
}

fn get_rocket_client() -> Client {
    let database_url = get_database_url();
    env::set_var("JWT_SECRET", "SECRET");
    Client::new(server::get_server(database_url)).expect("valid rocket instance")
}

#[test]
fn test_get_graphql_playground() {
    // given
    let client = get_rocket_client();

    // when
    let mut response = client.get("/").dispatch();

    // then
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    assert_eq!(
        response
            .body_string()
            .map(|body| body.contains("Playground")),
        Some(true)
    );
}

#[test]
fn test_api_version() {
    // given
    let client = get_rocket_client();

    // when
    let mut response = client
        .post("/graphql")
        .header(ContentType::JSON)
        .body("{\"query\":\"{apiVersion}\"}")
        .dispatch();

    // then
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(String::from("{\"data\":{\"apiVersion\":\"1.0\"}}"))
    );
}
