# Parfait

<p align="center">
    <img src="logo/logo.png" width="300" height="300" />
</p>

Parfait (/pɑːrˈfeɪ/ par-FAY) is the most lightweight web application framework. It is designed to make getting started quick and easy, with the ability to scale up to complex applications.

Parfait offers suggestions, but doesn't enforce any dependencies or project layout. It is up to the developer to choose the tools and libraries they want to use.

## Goal

The goal of Parfait is to provide a lightweight yet powerful solution for building web applications in Rust. While there are established frameworks like Rocket and Actix available, Parfait aims to offer a unique approach tailored to specific use cases and preferences.

### Macro-Based Approach

Parfait utilizes macros for defining `get` and `post` endpoints, etc. This decision was made to simplify the process of defining HTTP routes and handling requests. By using macros, developers can define routes in a concise and readable manner, reducing boilerplate code and improving code maintainability. Additionally, macros allow for compile-time validation of route definitions, catching errors early in the development process. Overall, the use of macros enhances developer productivity and contributes to the framework's goal of providing a lightweight and developer-friendly solution for building web applications in Rust.

### Comparison with Rocket

[Rocket](https://rocket.rs/) is a feature-rich web framework for Rust known for its ease of use and extensive capabilities. It provides a robust set of features out of the box, including routing, request parsing, and response generation, making it suitable for a wide range of web applications.

In comparison, Parfait takes a more minimalist approach, focusing on simplicity and flexibility. While Rocket excels in providing a comprehensive set of features, Parfait prioritizes lightweightness and customization. It aims to provide developers with more control over the components they use, allowing for greater flexibility in building web applications.

### Comparison with Actix

[Actix](https://actix.rs/) is a high-performance, actor-based framework for building concurrent and scalable web applications in Rust. It leverages the actor model to achieve high concurrency and asynchronous processing, making it suitable for applications with demanding performance requirements.

Unlike Actix, Parfait does not adopt the actor model and does not prioritize achieving the same level of concurrency and scalability. Instead, it focuses on simplicity and ease of use while still providing sufficient performance for most web applications. Parfait is designed to be approachable for developers new to Rust web development, offering straightforward abstractions and clear documentation.

## How to use

1. Input and output result:

```rust
// Define a handler for the input form
get!("/", home_handler => r#"src\input.html"#, "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n");

// Define a handler for the result page
post!("/result", result_handler => "src/output.html", "text/html"); // For HTML response
...
get_handler: Some(|path, query| home_handler(path, query, None)),
post_handler: Some(|path, query, body| result_handler(path, query, Some(body))),
```

**Note** fields in the Handler can accept None type. For example:

```rust
....
let handler = Handler {
    get_handler: Some(home_handler),
    post_handler: None,
};
```

More details can be found [here](examples/test/test.rs)

2. Loop:

```rust
post!("/loop", result_handler => r#"examples\test5\loop.html"#, "text/html");
...
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
```

More details can be found [here](examples/test5/test5.rs)

3. Using URL path:

```rust
// Define a handler for the input form
get!("/hello/", home_handler => r#"src\input.html"#, "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n");

// Define a handler for the result page
post!("/hello/result", result_handler => r#"src\output.html"#, "text/html");
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

More details can be found [here](examples/test4/test4.rs)

## Features

✅ post

✅ get

✅ put

✅ Handling get/post requests

✅ Compatibility with third-party libraries such as serde for JSON

✅ Error handling

✅ JSON

✅ Possibility to work with query

## Contributing

Contributions are absolutely, positively welcome and encouraged! Contributions
come in many forms. You could:

  1. Submit a feature request or bug report as an [issue].
  2. Ask for improved documentation as an [issue].
  3. Comment on issues that require feedback.
  4. Contribute code via [pull requests].

[issue]: https://github.com/ladroid/Parfait/issues
[pull requests]: https://github.com/ladroid/Parfait/pulls

## License

Parfait is licensed under Apache license version 2.0. See [LICENSE](https://github.com/ladroid/Parfait/blob/main/LICENSE) file.