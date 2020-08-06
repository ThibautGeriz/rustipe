extern crate recipes_backend;
extern crate serde_json;

use self::recipes_backend::infrastructure::web::jwt::decode_header;
use self::recipes_backend::infrastructure::web::server;

use diesel::pg::PgConnection;
use diesel::Connection;
use diesel::RunQueryDsl;
use rocket::http::{ContentType, Status};
use rocket::local::Client;
use serde_json::Value;
use std::error::Error;

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
    let url = get_database_url();
    env::set_var("JWT_SECRET", "SECRET");
    Client::new(server::get_server(url)).expect("valid rocket instance")
}

pub fn establish_connection() -> PgConnection {
    let database_url = get_database_url();
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn clean_db(connexion: &PgConnection) -> Result<(), Box<dyn Error>> {
    use self::recipes_backend::infrastructure::sql::schema::users::dsl::users;
    diesel::delete(users).execute(connexion)?;
    Ok(())
}

#[test]
fn test_signup() {
    // given
    let connexion = establish_connection();
    let client = get_rocket_client();

    // when
    let mut response = client
        .post("/graphql")
        .header(ContentType::JSON)
        .body(r#"{"query":"mutation {  signup(email: \"thibaut@gery.com\", password: \"toto\")}"}"#)
        .dispatch();

    // then
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let uuid = decode_header(&body["data"]["signup"].as_str().unwrap());
    assert_eq!(uuid.is_ok(), true);
    clean_db(&connexion).unwrap();
}

#[test]
fn test_login_success() {
    // given
    let connexion = establish_connection();
    let client = get_rocket_client();
    let mut response = client
        .post("/graphql")
        .header(ContentType::JSON)
        .body(
            r#"{"query":"mutation {  signup(email: \"thibaut2@gery.com\", password: \"toto2\")}"}"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let uuid_inserted = decode_header(&body["data"]["signup"].as_str().unwrap()).unwrap();

    // when
    let mut response = client
        .post("/graphql")
        .header(ContentType::JSON)
        .body(
            r#"{"query":"mutation {  signin(email: \"thibaut2@gery.com\", password: \"toto2\")}"}"#,
        )
        .dispatch();

    // then
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    let uuid = decode_header(&body["data"]["signin"].as_str().unwrap());
    assert_eq!(uuid.is_ok(), true);
    assert_eq!(uuid.unwrap(), uuid_inserted);
    clean_db(&connexion).unwrap();
}

#[test]
fn test_login_bad_password() {
    // given
    let connexion = establish_connection();
    let client = get_rocket_client();
    let response = client
        .post("/graphql")
        .header(ContentType::JSON)
        .body(
            r#"{"query":"mutation {  signup(email: \"thibaut3@gery.com\", password: \"toto3\")}"}"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // when
    let mut response = client
        .post("/graphql")
        .header(ContentType::JSON)
        .body(
            r#"{"query":"mutation {  signin(email: \"thibaut3@gery.com\", password: \"toto2\")}"}"#,
        )
        .dispatch();

    // then
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    assert_eq!(body.get("data"), Some(&Value::Null));

    clean_db(&connexion).unwrap();
}
