extern crate recipes_backend;
use self::recipes_backend::infrastructure::web::server;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    server::start_server(database_url)
}
