mod handler;
mod router;
pub mod server;

use server::Server;

fn main() {
    let server = server::Server::new("127.0.0.1:3000");
    server.run();
}
