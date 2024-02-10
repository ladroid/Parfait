use std::io;
use tokio::net::TcpListener;

extern crate parfait;
use parfait::*;

// Define a handler for the input form
get!("/", home_handler => r#"examples\input.html"#, "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n");

// Define a handler for the result page
post!("/result", result_handler => "examples/output.html", |file: &str, body: &str| {
    // Parse the body of the POST request and return the result
    let input = body.split("=").nth(1).unwrap_or("");
    let result = file.replace("{{ input }}", input);
    Some(result)
}, "text/html"); // For HTML response

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on port 8080...");

    let handler = Handler {
        get_handler: Some(home_handler),
        post_handler: Some(result_handler),
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