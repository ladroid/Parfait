use std::io;
extern crate parfait;
use parfait::*;

get!("/", home_handler => ContentType::String("Hello, world"), "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n");

#[tokio::main]
async fn main() -> io::Result<()> { 
    let handler = Handler {
        get_handler: Some(|path, query| home_handler(path, query, None)),
        post_handler: None,
        put_handler: None,
        delete_handler: None,
    };
    run("127.0.0.1", 8080, handler).await
}
