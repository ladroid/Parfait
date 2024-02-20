use std::io;

extern crate parfait;
use parfait::*;

get!("/", home_handler => ContentType::File(r#"examples\test\input.html"#), "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n");

post!("/result", result_handler => "examples/test/output.html", "text/html"); // For HTML response

#[tokio::main]
async fn main() -> io::Result<()> {
    let handler = Handler {
        get_handler: Some(|path, query| home_handler(path, query, None)),
        post_handler: Some(|path, query, body| result_handler(path, query, Some(body))),
        put_handler: None,
        delete_handler: None,
    };
    run("127.0.0.1", 8080, handler).await
}