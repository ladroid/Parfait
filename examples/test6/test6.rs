use std::io;
use tokio::net::TcpListener;

extern crate parfait;
use parfait::*;

get!("/", home_handler => ContentType::String("Hello, world"), "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n");

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on port 8080...");

    let handler = Handler {
        get_handler: Some(|path, query| home_handler(path, query, None)),
        post_handler: None,
        put_handler: None,
    };

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(err) = handle_client(stream, &handler).await {
                eprintln!("Error handling client: {:?}", err);
            }
        });
    }
}