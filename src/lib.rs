#![macro_use]

use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[macro_export]
macro_rules! get {
    ($path:expr, $name:ident => $filename:expr, $headers:expr) => {
        pub fn $name(path: &str) -> Option<String> {
            if path == $path {
                match std::fs::File::open($filename) {
                    Ok(mut html) => {
                        use std::io::Read;
                        let mut content = String::new();
                        html.read_to_string(&mut content).unwrap();
                        Some(format!("{}{}", $headers, content))
                    },
                    Err(_) => None,
                }
            } else {
                None
            }
        }
    };
}

#[macro_export]
macro_rules! post {
    ($path:expr, $name:ident => $filename:expr, $handler:expr, $content_type:expr) => {
        pub fn $name(path: &str, body: &str) -> Option<String> {
            if path == $path {
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

#[macro_export]
macro_rules! put {
    ($path:expr, $name:ident => $filename:expr, $handler:expr, $content_type:expr) => {
        pub fn $name(path: &str, body: &str) -> Option<String> {
            if path == $path {
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

#[derive(Copy, Clone)]
pub struct Handler {
    pub get_handler: Option<fn(&str) -> Option<String>>,
    pub post_handler: Option<fn(&str, &str) -> Option<String>>,
    pub put_handler: Option<fn(&str, &str) -> Option<String>>, // New put handler
}

impl Handler {
    pub fn handle_request(&self, request: &str) -> Option<String> {
        println!("Received request: {}", request); // Debug print
        let mut lines = request.lines();
        let request_line = lines.next()?;
        let mut parts = request_line.split(' ');

        match (parts.next(), parts.next()) {
            (Some(method), Some(path)) => {
                println!("Method: {}, Path: {}", method, path); // Debug print
                match method {
                    "GET" => {
                        if let Some(handler) = self.get_handler {
                            handler(path)
                        } else {
                            Some("HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned())
                        }
                    }
                    "POST" => {
                        if let Some(handler) = self.post_handler {
                            let mut body = "";
                            for line in lines {
                                if line == "\r" {
                                    break;
                                } else {
                                    body = line;
                                }
                            }
                            println!("Body: {}", body); // Debug print
                            handler(path, body)
                        } else {
                            Some("HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned())
                        }
                    }
                    "PUT" => { // New branch for PUT method
                        if let Some(handler) = self.put_handler {
                            let mut body = "";
                            for line in lines {
                                if line == "\r" {
                                    break;
                                } else {
                                    body = line;
                                }
                            }
                            println!("Body: {}", body); // Debug print
                            handler(path, body)
                        } else {
                            Some("HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned())
                        }
                    }
                    _ => Some("HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\n".to_owned()),
                }
            }
            _ => Some("HTTP/1.1 400 BAD REQUEST\r\n\r\n".to_owned()),
        }
    }
}

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

// Function to generate JSON response
pub fn generate_json_response(data: serde_json::Value) -> String {
    serde_json::to_string(&data).unwrap_or_default()
}