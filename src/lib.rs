#![allow(unused_variables)]
#![allow(dead_code)]
#![macro_use]

use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Enum to represent the content type for GET macro
pub enum ContentType<'a> {
    File(&'a str),
    String(&'a str),
}

/// Defines a GET endpoint.
///
/// # Usage
/// ```rust
/// get!(path, handler_function => filename, headers)
/// ```
///
/// # Description
/// Defines a GET endpoint. When a GET request matches the specified `path`, the `handler_function` is invoked to generate a response based on the content of the specified `filename`. Additional headers can be included in the response.
///
/// # Parameters
/// - `path`: The path prefix for which the endpoint is defined.
/// - `handler_function`: The name of the function to handle the request.
/// - `filename`: The name of the file containing the content for the response.
/// - `headers`: Additional headers to include in the response.
#[macro_export]
macro_rules! get {
    ($path:expr, $name:ident => $content:expr, $headers:expr) => {
        pub fn $name(path: &str, query: Option<&str>, _: Option<&str>) -> Option<String> {
            if path.starts_with($path) {
                let mut content = match $content {
                    ContentType::File(filename) => {
                        match std::fs::read_to_string(filename) {
                            Ok(content) => content,
                            Err(_) => return None,
                        }
                    }
                    ContentType::String(str_content) => str_content.to_owned(),
                };

                // Replace query parameters if they exist
                if let Some(query_params) = query {
                    for param in query_params.split('&') {
                        let parts: Vec<&str> = param.split('=').collect();
                        if parts.len() == 2 {
                            let formatted_query = format!("{{{{ {} }}}}", parts[0]);
                            content = content.replace(&formatted_query, parts[1]);
                        }
                    }
                }

                Some(format!("{}{}", $headers, content))
            } else {
                None
            }
        }
    };
}

/// Defines a POST endpoint.
///
/// # Usage
/// ```rust
/// post!(path, handler_function => filename, content_type)
/// ```
///
/// # Description
/// Defines a POST endpoint. When a POST request matches the specified `path`, the `handler_function` is invoked to generate a response based on the content of the specified `filename`. The `content_type` parameter specifies the MIME type of the response.
///
/// # Parameters
/// - `path`: The path prefix for which the endpoint is defined.
/// - `handler_function`: The name of the function to handle the request.
/// - `filename`: The name of the file containing the content for the response.
/// - `content_type`: The MIME type of the response.
#[macro_export]
macro_rules! post {
    ($path:expr, $name:ident => $filename:expr, $content_type:expr) => {
        pub fn $name(path: &str, query: Option<&str>, body: Option<&str>) -> Option<String> {
            if path.starts_with($path) {
                match std::fs::File::open($filename) {
                    Ok(mut file) => {
                        use std::io::Read;
                        let mut content = String::new();
                        file.read_to_string(&mut content).unwrap();
                        
                        // Replace query parameters if they exist
                        if let Some(query_params) = query {
                            for param in query_params.split('&') {
                                let parts: Vec<&str> = param.split('=').collect();
                                if parts.len() == 2 {
                                    let formatted_query = format!("{{{{ {} }}}}", parts[0]);
                                    content = content.replace(&formatted_query, parts[1]);
                                }
                            }
                        }
                        
                        // Replace body parameters if they exist
                        if let Some(body_content) = body {
                            for param in body_content.split('&') {
                                let parts: Vec<&str> = param.split('=').collect();
                                if parts.len() == 2 {
                                    let formatted_body = format!("{{{{ {} }}}}", parts[0]);
                                    content = content.replace(&formatted_body, parts[1]);
                                }
                            }
                        }
                        
                        Some(format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n{}", $content_type, content))
                    },
                    Err(_) => None,
                }
            } else {
                None
            }
        }
    };
}

/// Defines a PUT endpoint.
///
/// # Usage
/// ```rust
/// put!(path, handler_function => filename, handler, content_type)
/// ```
///
/// # Description
/// Defines a PUT endpoint. When a PUT request matches the specified `path`, the `handler_function` is invoked to generate a response based on the content of the specified `filename`. The `handler` parameter allows specifying a custom handler function for processing the request body. The `content_type` parameter specifies the MIME type of the response.
///
/// # Parameters
/// - `path`: The path prefix for which the endpoint is defined.
/// - `handler_function`: The name of the function to handle the request.
/// - `filename`: The name of the file containing the content for the response.
/// - `handler`: A custom handler function for processing the request body.
/// - `content_type`: The MIME type of the response.
#[macro_export]
macro_rules! put {
    ($path:expr, $name:ident => $filename:expr, $handler:expr, $content_type:expr) => {
        pub fn $name(path: &str, body: &str) -> Option<String> {
            if path.starts_with($path) {
                match std::fs::File::open($filename) {
                    Ok(mut file) => {
                        use std::io::Read;
                        let mut content = String::new();
                        file.read_to_string(&mut content).unwrap();
                        let result = $handler(&content, body);
                        match result {
                            Some(result) => Some(format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n{}", $content_type, result)),
                            None => Some(format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n", $content_type)),
                        }
                    },
                    Err(_) => None,
                }
            } else {
                None
            }
        }
    };
}

/// Defines a DELETE endpoint for removing data elements from a JSON file or any text-based content.
///
/// # Usage
/// ```rust
/// delete!(path, handler_function => content_type, key)
/// ```
///
/// # Description
/// Defines a DELETE endpoint. When a DELETE request matches the specified `path`, the `handler_function` is invoked to remove an element identified by `key` from the specified content source. The `content_type` specifies whether the operation is on a JSON file or another type of text content. The operation can also be conditioned on query parameters.
///
/// # Parameters
/// - `path`: The path prefix for which the endpoint is defined.
/// - `handler_function`: The name of the function to handle the request.
/// - `content_type`: The type of content being modified (`ContentType::File` for files, `ContentType::String` for in-memory strings).
/// - `key`: The key or identifier of the element to be removed. For JSON, this would be the property name.
#[macro_export]
macro_rules! delete {
    ($path:expr, $name:ident => $content_type:expr, $key:expr) => {
        pub fn $name(path: &str, query: Option<&str>) -> Option<String> {
            if path.starts_with($path) {
                let key_to_remove = if let Some(query_params) = query {
                    query_params.split('&')
                        .find_map(|param| {
                            let parts: Vec<&str> = param.split('=').collect();
                            if parts.len() == 2 && parts[0] == "key" {
                                Some(parts[1])
                            } else {
                                None
                            }
                        })
                        .unwrap_or($key)
                } else {
                    $key
                };

                match $content_type {
                    ContentType::File(filename) => {
                        let mut file_content = match std::fs::read_to_string(filename) {
                            Ok(content) => content,
                            Err(_) => return Some("HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\n\r\nFile not found.".to_owned()),
                        };

                        // Specific handling for JSON files
                        if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&file_content) {
                            json.as_object_mut().unwrap().remove(key_to_remove);
                            file_content = serde_json::to_string(&json).unwrap();
                            std::fs::write(filename, file_content).unwrap();
                            Some(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nElement '{}' removed successfully.", key_to_remove))
                        } else {
                            // Fallback for non-JSON content, demonstrating intent
                            Some("HTTP/1.1 501 NOT IMPLEMENTED\r\nContent-Type: text/plain\r\n\r\nDeletion from non-JSON content not implemented.".to_owned())
                        }
                    },
                    ContentType::String(str_content) => {
                        // Demonstration for in-memory string content, not implemented
                        Some("HTTP/1.1 501 NOT IMPLEMENTED\r\nContent-Type: text/plain\r\n\r\nDeletion from in-memory content not implemented.".to_owned())
                    },
                }
            } else {
                None
            }
        }
    };
}

/// Represents a handler for processing HTTP requests.
///
/// Contains optional functions for handling GET, POST, and PUT requests.
#[derive(Copy, Clone)]
pub struct Handler {
    pub get_handler: Option<fn(&str, Option<&str>) -> Option<String>>,
    pub post_handler: Option<fn(&str, Option<&str>, &str) -> Option<String>>,
    pub put_handler: Option<fn(&str, &str) -> Option<String>>,
    pub delete_handler: Option<fn(&str, Option<&str>) -> Option<String>>,
}

impl Handler {
    pub fn handle_request(&self, request: &str) -> Option<String> {
        println!("Received request: {}", request); // Debug print
        let mut lines = request.lines();
        let request_line = lines.next()?;
        let mut parts = request_line.split(' ');

        let (method, path, query) = match (parts.next(), parts.next()) {
            (Some(method), Some(path)) => {
                let query_start = path.find('?');
                let (path_without_query, query) = match query_start {
                    Some(i) => (&path[..i], Some(&path[i + 1..])),
                    None => (path, None),
                };
                (method, path_without_query, query)
            }
            _ => return Some("HTTP/1.1 400 BAD REQUEST\r\n\r\n".to_owned()),
        };

        match method {
            "GET" => {
                if let Some(handler) = self.get_handler {
                    handler(path, query)
                } else {
                    Some("HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned())
                }
            }
            "POST" => {
                let mut body = "";
                for line in lines {
                    if line == "\r" {
                        break;
                    } else {
                        body = line;
                    }
                }
                if let Some(handler) = self.post_handler {
                    handler(path, query, body)
                } else {
                    Some("HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned())
                }
            }
            "PUT" => {
                if let Some(handler) = self.put_handler {
                    let mut body = "";
                    for line in lines {
                        if line == "\r" {
                            break;
                        } else {
                            body = line;
                        }
                    }
                    handler(path, body)
                } else {
                    Some("HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned())
                }
            }
            "DELETE" => {
                if let Some(handler) = self.delete_handler {
                    handler(path, query)
                } else {
                    Some("HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned())
                }
            }
            _ => Some("HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\n".to_owned()),
        }
    }
}

/// Asynchronously handles an incoming TCP stream containing an HTTP request.
///
/// Parses the request, invokes the appropriate handler function based on the request method, generates a response, and sends it back over the stream.
pub async fn handle_client(mut stream: TcpStream, handler: &Handler) -> io::Result<()> {
    let mut buffer = [0; 1024];
    let mut request = String::new(); // String to store the entire request
    let mut body = String::new(); // String to store the request body

    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            break; // End of stream
        }

        request.push_str(&String::from_utf8_lossy(&buffer[..n])); // Append buffer to request

        // Check if the end of the request body is reached
        if let Some(i) = request.find("\r\n\r\n") {
            body.push_str(&request[i + 4..]); // Append body to the body string
            break;
        }
    }

    let response = match handler.handle_request(&request) {
        Some(response) => response,
        None => "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned(),
    };

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}

/// Extracts a cookie value from an HTTP request.
pub fn get_cookie(request: &str, name: &str) -> Option<String> {
    let cookie_header = request.lines().find(|line| line.starts_with("Cookie: "))?;

    let cookie_str = cookie_header.trim_start_matches("Cookie: ").trim();
    let cookies: Vec<_> = cookie_str.split(';').collect();

    for cookie in cookies {
        let parts: Vec<_> = cookie.trim().split('=').collect();
        if parts.len() == 2 && parts[0] == name {
            return Some(parts[1].to_owned());
        }
    }

    None
}

/// Parses a JSON string into a `serde_json::Value` object.
// Function to parse JSON
pub fn parse_json(body: &str) -> Option<serde_json::Value> {
    match serde_json::from_str(body) {
        Ok(value) => Some(value),
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            None
        }
    }
}

/// Converts a `serde_json::Value` object into a JSON string.
// Function to generate JSON response
pub fn generate_json_response(data: serde_json::Value) -> String {
    serde_json::to_string(&data).unwrap_or_default()
}

pub async fn run(addr: &str, port: u16, handler: Handler) -> io::Result<()> {
    let address = format!("{}:{}", addr, port);
    let listener = tokio::net::TcpListener::bind(&address).await?;
    println!("Server listening on {}", address);
    
    let handler = std::sync::Arc::new(handler); // Wrap handler in an Arc for shared ownership

    loop {
        let (stream, _) = listener.accept().await?;
        let handler_clone = handler.clone(); // Clone the Arc to get a new reference for the new task

        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, &handler_clone).await {
                eprintln!("Failed to handle client: {}", e);
            }
        });
    }
}

/// Middleware support for pre and post request processing.
pub trait Middleware: Send + Sync {
    fn before(&self, request: &str) -> Option<String>;
    fn after(&self, response: &str) -> Option<String>;
}

/// Basic logging middleware example.
pub struct LoggingMiddleware;

impl Middleware for LoggingMiddleware {
    fn before(&self, request: &str) -> Option<String> {
        println!("Received request: {}", request);
        None
    }

    fn after(&self, response: &str) -> Option<String> {
        println!("Sending response: {}", response);
        None
    }
}

/// Support for routing with parameter extraction.
#[macro_export]
macro_rules! route {
    ($method:expr, $path:expr, $handler:expr) => {
        if $method == "GET" && $path.starts_with($path) {
            // Extract parameters from path and pass them to the handler
            let params: Vec<&str> = $path.split('/').collect();
            $handler(params)
        }
    };
}

/// JSON Web Token (JWT) authentication middleware.
pub struct AuthMiddleware {
    valid_tokens: Vec<String>, // Could be a more complex structure for scalability
}

impl AuthMiddleware {
    pub fn new(valid_tokens: Vec<String>) -> AuthMiddleware {
        AuthMiddleware {
            valid_tokens,
        }
    }
}

impl Middleware for AuthMiddleware {
    fn before(&self, request: &str) -> Option<String> {
        let token = request.lines()
            .find(|line| line.starts_with("Authorization: Bearer "))
            .and_then(|header| Some(header.replace("Authorization: Bearer ", "")));

        match token {
            Some(t) if self.valid_tokens.contains(&t) => None,
            _ => Some("HTTP/1.1 401 Unauthorized\r\n\r\n".to_owned()),
        }
    }

    fn after(&self, response: &str) -> Option<String> {
        None
    }
}

/// Support for static file serving.
pub fn serve_static(path: &str) -> Option<String> {
    // Serve files from a designated static directory
    let static_file_path = format!("static/{}", path);
    match std::fs::read_to_string(static_file_path) {
        Ok(content) => Some(format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", content)),
        Err(_) => Some("HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned()),
    }
}

/// Integrating middleware into the request handling process.
pub async fn handle_client_with_middleware(mut stream: TcpStream, handler: &Handler, middleware: &dyn Middleware) -> io::Result<()> {
    // Similar to handle_client, but with middleware invocation
    let mut buffer = [0; 1024];
    let mut request = String::new();

    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 { break; }

        request.push_str(&String::from_utf8_lossy(&buffer[..n]));
        if request.contains("\r\n\r\n") { break; }
    }

    // Invoke the before middleware function
    if let Some(response) = middleware.before(&request) {
        stream.write_all(response.as_bytes()).await?;
        return Ok(());
    }

    let response = handler.handle_request(&request).unwrap_or_else(|| "HTTP/1.1 500 Internal Server Error\r\n\r\n".to_owned());

    // Invoke the after middleware function
    let response = middleware.after(&response).unwrap_or(response);

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}

/// Modified server run function that accepts middleware.
pub async fn run_with_middleware(addr: &str, port: u16, handler: Handler, middleware: std::sync::Arc<dyn Middleware>) -> io::Result<()> {
    let address = format!("{}:{}", addr, port);
    let listener = tokio::net::TcpListener::bind(&address).await?;
    println!("Server listening on {}", address);

    let handler = std::sync::Arc::new(handler);

    loop {
        let (stream, _) = listener.accept().await?;
        let handler_clone = handler.clone();
        let middleware_clone = middleware.clone(); // Clone Arc for the new task

        tokio::spawn(async move {
            if let Err(e) = handle_client_with_middleware(stream, &handler_clone, &*middleware_clone).await {
                eprintln!("Failed to handle client: {}", e);
            }
        });
    }
}