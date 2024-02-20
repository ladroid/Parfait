use std::io;

extern crate parfait;
use parfait::*;

post!("/path", handle_post => r#"examples\test4\file.json"#, "application/json");

#[tokio::main]
async fn main() -> io::Result<()> { 
    let handler = Handler {
        get_handler: None,
        post_handler: Some(|_, _, _| {
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
        delete_handler: None,
    };
    run("127.0.0.1", 8080, handler).await
}
