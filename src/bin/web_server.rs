extern crate recipes_backend;

use self::recipes_backend::infrastructure::web::server;

fn main() {
    server::start_server()
}
