extern crate parfait;
use parfait::*;

get!("/", get_index => ContentType::File(r#"examples\test3\index.html"#), "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n");

post!("/submit", post_submit => r#"examples\test3\submit.html"#, "application/json");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let handler = Handler {
        get_handler: Some(|path, query| get_index(path, query, None)),
        post_handler: Some(|path, query, body| post_submit(path, query, Some(body))),
        put_handler: None,
        delete_handler: None,
    };
    run("127.0.0.1", 8080, handler).await?;
    Ok(())
}
