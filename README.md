# Parfait

Parfait is the most lightweight web application framework. It is designed to make getting started quick and easy, with the ability to scale up to complex applications.

Parfait offers suggestions, but doesn't enforce any dependencies or project layout. It is up to the developer to choose the tools and libraries they want to use.

## How to use

1. Input and output result:

```rust
// Define a handler for the input form
get!("/", home_handler => r#"src\input.html"#, "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n");

// Define a handler for the result page
post!("/result", result_handler => "src/output.html", |file: &str, body: &str| {
    // Parse the body of the POST request and return the result
    let input = body.split("=").nth(1).unwrap_or("");
    let result = file.replace("{{ input }}", input);
    Some(result)
}, "text/html"); // For HTML response
```

**Note** fields in the Handler can accept None type. For example:

```rust
....
let handler = Handler {
    get_handler: Some(home_handler),
    post_handler: None,
};
```

2. Loop:

```rust
post!("/loop", result_handler => r#"src\loop.html"#, |file: &str, body: &str| {
    let items = vec!["Item 1", "Item 2", "Item 3"];
    let mut result = String::new();
    let mut in_for_loop = false;
    for line in file.lines() {
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
    Some(result)
}, "text/html");
```

3. Using URL path:

```rust
// Define a handler for the input form
get!("/hello/", home_handler => r#"src\input.html"#, "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n");

// Define a handler for the result page
post!("/hello/result", result_handler => r#"src\output.html"#, |file: &str, body: &str| {
    // Parse the body of the POST request and return the result
    let input = body.split("=").nth(1).unwrap_or("");
    let result = file.replace("{{ input }}", input);
    result
});
```

4. JSON

```rust
post!("/path", handle_post => r#"src\file.json"#, |_, _| {
    match std::fs::read_to_string("src\\file.json") {
        Ok(content) => {
            if let Some(json_data) = paprika::parse_json(&content) {
                let response_json = paprika::generate_json_response(json_data);
                Some(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", response_json))
            } else {
                Some("HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\nFailed to parse JSON".to_owned())
            }
        },
        Err(_) => Some("HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\nFailed to read file".to_owned()),
    }
});
```

## Features

✅ post

✅ get

✅ Handling get/post requests

✅ Compatibility with third-party libraries such as serde for JSON

✅ Error handling

✅ JSON