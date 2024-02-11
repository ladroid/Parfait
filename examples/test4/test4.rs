use std::io;
use tokio::net::TcpListener;

extern crate parfait;
use parfait::*;

post!("/path", handle_post => r#"examples\test4\file.json"#, "application/json");

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on port 8080...");

    let handler = Handler {
        get_handler: None,
        post_handler: Some(|path, _, _| {
            // Read the file content
            match std::fs::read_to_string("examples\\test4\\file.json") {
                Ok(content) => {
                    // Parse the JSON data
                    if let Some(json_data) = parse_json(&content) {
                        // Generate JSON response
                        let response_json = generate_json_response(json_data);
                        Some(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", response_json))
                    } else {
                        Some("HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\nFailed to parse JSON".to_owned())
                    }
                },
                Err(_) => Some("HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\nFailed to read file".to_owned()),
            }
        }),
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
