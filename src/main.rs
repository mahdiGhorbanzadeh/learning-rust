use http::request::Request;
use server::Server;

mod http;
mod server;
fn main() {
    let url: &str = "127.0.0.1:8081";

    let url_string: String = url.to_string();

    let server: Server = Server::new(url_string);

    server.run();
}
