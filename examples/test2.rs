use tokio::net::TcpListener;

extern crate parfait;
use parfait::*;

get!("/", get_index => r#"examples\index.html"#, "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n");

post!("/submit", post_submit => r#"examples\submit.html"#, |file: &str, body: &str| {
    // Parse the body of the POST request and return the result
    let input = body.split("=").nth(1).unwrap_or("");
    let result = file.replace("{{ data }}", input);
    Some(result)
}, "application/json");

put!("/update", put_update => r#"examples\update.html"#, |file: &str, body: &str| {
    // Parse the body of the POST request and return the result
    let input = body.split("=").nth(1).unwrap_or("");
    let result = file.replace("{{ data }}", input);
    Some(result)
}, "application/json");


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on port 8080...");

    let handler = Handler {
        get_handler: Some(get_index),
        post_handler: Some(post_submit),
        put_handler: Some(put_update), // Example PUT handler
    };

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, &handler).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}