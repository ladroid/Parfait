use std::io;

extern crate parfait;
use parfait::*;

post!("/loop", result_handler => r#"examples\test5\loop.html"#, "text/html");

#[tokio::main]
async fn main() -> io::Result<()> { 
    let handler = Handler {
        get_handler: None,
        post_handler: Some(|_, _, _| {
            // Read the file content
            match std::fs::read_to_string("examples\\test5\\loop.html") {
                Ok(file_content) => {
                    let items = vec!["Item 1", "Item 2", "Item 3"];
                    let mut result = String::new();
                    let mut in_for_loop = false;
                    
                    for line in file_content.lines() {
                        if line.contains("{% for item in items %}") {
                            in_for_loop = true;
                            for item in &items {
                                result.push_str(&line.replace("{% for item in items %}", &format!("{}", item)));
                                result.push_str("\n");
                            }
                        } else if in_for_loop && line.contains("{% endfor %}") {
                            in_for_loop = false;
                        }
                    }
                    Some(format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", result))
                },
                Err(_) => Some("HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\nFailed to read file".to_owned()),
            }
        }),
        put_handler: None,
        delete_handler: None,
    };
    run("127.0.0.1", 8080, handler).await
}
