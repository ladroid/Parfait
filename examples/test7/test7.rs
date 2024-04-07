use std::io;
extern crate parfait;
use parfait::*;

get!("/", home_handler => ContentType::String("Hello, world"), "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n");

#[tokio::main]
async fn main() -> io::Result<()> {
    // Example token for demonstration. In a real application, secure and dynamic token retrieval should be used.
    let valid_tokens = vec!["valid_token".to_string()]; // This could be dynamically loaded

    // Define the address and port the server will run on
    let addr = "127.0.0.1";
    let port = 8080;

    // Create a handler with some example routes
    let handler = Handler {
        get_handler: Some(|path, query| home_handler(path, query, None)),
        post_handler: None,
        put_handler: None,
        delete_handler: None,
    };

    // Initialize middleware
    let auth_middleware = AuthMiddleware::new(valid_tokens);

    // Run the server with the specified handler and middleware
    run_with_middleware(addr, port, handler, std::sync::Arc::new(auth_middleware)).await
}