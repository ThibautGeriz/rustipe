extern crate recipes_backend;
extern crate serde_json;

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
    println!("{:?}", url);
    Client::new(server::get_server(url)).expect("valid rocket instance")
}

pub fn establish_connection() -> PgConnection {
    let database_url = get_database_url();
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn clean_db(connexion: &PgConnection) -> Result<(), Box<dyn Error>> {
    use self::recipes_backend::infrastructure::sql::schema::ingredients::dsl::ingredients;
    use self::recipes_backend::infrastructure::sql::schema::instructions::dsl::instructions;
    use self::recipes_backend::infrastructure::sql::schema::recipes::dsl::recipes;
    use self::recipes_backend::infrastructure::sql::schema::users::dsl::users;

    diesel::delete(ingredients).execute(connexion)?;
    diesel::delete(instructions).execute(connexion)?;
    diesel::delete(recipes).execute(connexion)?;
    diesel::delete(users).execute(connexion)?;
    Ok(())
}

fn init_with_users(connexion: &PgConnection) -> Result<(), Box<dyn Error>> {
    use self::recipes_backend::infrastructure::sql::models::*;
    use self::recipes_backend::infrastructure::sql::schema::users;

    let new_user_1 = NewUser {
        id: "user_id_1",
        email: "email1",
        password_hash: "password",
    };
    let new_user_2 = NewUser {
        id: "user_id_2",
        email: "email2",
        password_hash: "password",
    };
    diesel::insert_into(users::table)
        .values(&vec![new_user_1, new_user_2])
        .get_result::<User>(connexion)
        .unwrap();
    Ok(())
}

#[test]
fn test_get_recipes_without_recipes() {
    // given
    let connexion = establish_connection();
    init_with_users(&connexion).unwrap();
    let client = get_rocket_client();

    // when
    let mut response = client
        .post("/graphql")
        .header(ContentType::JSON)
        .body(r#"{"query":"{\n  getMyRecipes(userId: \"user_id_1\") {\n    title\n    instructions\n    ingredients\n  }\n}\n"}"#)
        .dispatch();

    // then
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(String::from("{\"data\":{\"getMyRecipes\":[]}}"))
    );

    clean_db(&connexion).unwrap();
}

#[test]
fn test_add_recipe() {
    // given
    let connexion = establish_connection();
    init_with_users(&connexion).unwrap();
    let client = get_rocket_client();

    // when
    let mut response = client
        .post("/graphql")
        .header(ContentType::JSON)
        .body(r#"{"query":"mutation {\n  createRecipe(newRecipe: {title: \"my recipe\", userId: \"user_id_1\", instructions: [\"ins1\", \"ins2\"], ingredients: [\"ing1\", \"ing2\"]}) {\n    id\n   title ingredients\n    description\n    instructions\n  }\n}\n"}"#)
        .dispatch();

    // then
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    assert_eq!(body["data"]["createRecipe"]["title"], "my recipe");
    clean_db(&connexion).unwrap();
}

#[test]
fn test_get_recipes_with_2_recipes() {
    // given
    let connexion = establish_connection();
    init_with_users(&connexion).unwrap();
    let client = get_rocket_client();
    let response_recipe_1 = client
        .post("/graphql")
        .header(ContentType::JSON)
        .body(r#"{"query":"mutation {\n  createRecipe(newRecipe: {title: \"my recipe\", userId: \"user_id_1\", instructions: [\"ins1\", \"ins2\"], ingredients: [\"ing1\", \"ing2\"]}) {\n    id\n   title ingredients\n    description\n    instructions\n  }\n}\n"}"#)
        .dispatch();
    assert_eq!(response_recipe_1.status(), Status::Ok);
    let response_recipe_2 = client
        .post("/graphql")
        .header(ContentType::JSON)
        .body(r#"{"query":"mutation {\n  createRecipe(newRecipe: {title: \"my recipe 2\", userId: \"user_id_1\", instructions: [\"ins1\", \"ins2\"], ingredients: [\"ing1\", \"ing2\"]}) {\n    id\n   title ingredients\n    description\n    instructions\n  }\n}\n"}"#)
        .dispatch();
    assert_eq!(response_recipe_2.status(), Status::Ok);

    // when
    let response = client
        .post("/graphql")
        .header(ContentType::JSON)
        .body(r#"{"query":"{\n  getMyRecipes(userId: \"user_id_1\") {\n    title\n    instructions\n    ingredients\n  }\n}\n"}"#)
        .dispatch();

    // then
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    clean_db(&connexion).unwrap();
}

#[test]
fn test_get_single_recipe() {
    // given
    let connexion = establish_connection();
    init_with_users(&connexion).unwrap();
    let client = get_rocket_client();
    let mut response_recipe_1 = client
        .post("/graphql")
        .header(ContentType::JSON)
        .body(r#"{"query":"mutation {\n  createRecipe(newRecipe: {title: \"my recipe\", userId: \"user_id_2\", instructions: [\"ins1\", \"ins2\"], ingredients: [\"ing1\", \"ing2\"]}) {\n    id\n   title ingredients\n    description\n    instructions\n  }\n}\n"}"#)
        .dispatch();
    assert_eq!(response_recipe_1.status(), Status::Ok);
    let body: Value = serde_json::from_str(&response_recipe_1.body_string().unwrap()).unwrap();
    let id: &str = &body["data"]["createRecipe"]["id"].as_str().unwrap();

    // when
    let mut response = client
        .post("/graphql")
        .header(ContentType::JSON)
        .body(format!(
            r#"{{"query":"{{  getRecipe(id: \"{id}\") {{ id title }} }}"}}"#,
            id = id
        ))
        .dispatch();

    // then
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.body_string(),
        Some(String::from(format!(
            "{{\"data\":{{\"getRecipe\":{{\"id\":\"{id}\",\"title\":\"my recipe\"}}}}}}",
            id = id
        )))
    );

    clean_db(&connexion).unwrap();
}