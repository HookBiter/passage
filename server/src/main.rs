#[path = "server/server.rs"]
mod server;

use crate::server::Server;
use std::sync::Arc;

fn main() {
    println!("Hello, world!");
    let server = Arc::new(&Server {});
    server.start();
}
