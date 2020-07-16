use rocket::http::Method;
use rocket::{response::content, Rocket, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

use crate::infrastructure::web::graphql_schema::{Context, Mutation, Query, Schema};

pub fn get_server(database_url: String) -> Rocket {
    rocket::ignite()
        .manage(Context::new(database_url))
        .manage(Schema::new(Query, Mutation))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .attach(make_cors())
}

pub fn start_server(database_url: String) {
    get_server(database_url).launch();
}

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::playground_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::All;

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "content-type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}
