extern crate recipes_backend;

use self::recipes_backend::infrastructure::web::server;

use rocket::http::{ContentType, Status};
use rocket::local::Client;

#[test]
fn test_get_graphql_playground() {
    // given
    let client = Client::new(server::get_server()).expect("valid rocket instance");

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
    let client = Client::new(server::get_server()).expect("valid rocket instance");

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
