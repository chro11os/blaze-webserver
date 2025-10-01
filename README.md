🔥 Blaze Webserver

A blazingly fast, lightweight, and modern web server built from scratch in Rust, with a relentless focus on performance and memory safety.

<details>
<summary><strong>Table of Contents</strong></summary>
<ol>
<li><a href="#-project-status-work-in-progress-">Project Status</a></li>
<li><a href="#-why-blaze">Why Blaze?</a></li>
<li><a href="#-features">Features</a></li>
<li><a href="#-goals">Goals</a></li>
<li><a href="#-getting-started">Getting Started</a></li>
<li><a href="#-project-roadmap">Roadmap</a></li>
<li><a href="#-contributing">Contributing</a></li>
<li><a href="#-license">License</a></li>
</ol>
</details>
🚧 Project Status: Work in Progress 🚧

Blaze is currently under active development and is not yet ready for production use.

This project started as an effort to build a high-performance web server from the ground up, leveraging Rust's unique advantages for systems programming.
🤔 Why Blaze?

Blaze began as a deep dive into the fundamentals of web technologies through the lens of Rust. It's a journey to understand how high-performance servers work by building one from first principles. This project prioritizes learning, performance, and idiomatic Rust, aiming to create a server that is not only fast but also a clear and well-documented example of systems programming.
✨ Features

    Built in pure Rust: For memory safety, concurrency, and performance.

    Asynchronous Architecture: Built on tokio for non-blocking I/O to handle thousands of concurrent connections efficiently.

    HTTP/1.1 Parsing: Basic request parsing from raw TCP streams.

    Static File Serving: Capable of serving static files from a root directory.

    Planned - Thread Pool: To efficiently manage CPU-bound tasks.

    Planned - Extensible Middleware: A simple system to add custom logic to the request-response cycle.

🎯 Goals

    Raw Speed: Achieve a high requests-per-second count with low latency.

    Low Memory Footprint: Be as resource-efficient as possible.

    Security: Leverage Rust's type and memory safety to prevent common vulnerabilities.

    Learning: Serve as a comprehensive learning project for anyone interested in Rust and networking.

🚀 Getting Started
Prerequisites

    Rust toolchain (latest stable version recommended): https://rustup.rs/

Running the Server

    Clone the repository:

    git clone [https://github.com/your-username/blaze-webserver.git](https://github.com/your-username/blaze-webserver.git)
    cd blaze-webserver

    Run in development mode:

    cargo run

By default, the server will start on 127.0.0.1:8080 and serve files from a local public directory. You can test it by opening your browser and navigating to http://localhost:8080.
🗺️ Project Roadmap

    [ ] Core Features

        [x] Basic TCP Listener

        [ ] Robust HTTP/1.1 Request Parsing

        [ ] Response Serialization

        [ ] Concurrency using a Thread Pool

    [ ] Advanced Features

        [ ] Basic Routing

        [ ] Middleware Support

        [ ] Configuration from a file (.toml)

        [ ] TLS/HTTPS Support

        [ ] Logging

    [ ] Performance

        [ ] Benchmarking against other servers

        [ ] Performance optimizations

🤝 Contributing

Contributions are highly welcome! This project is a great place to learn about Rust and web technologies. Feel free to open an issue to discuss a new feature or submit a pull request.

As a fellow developer from the Philippines, I'm excited to see what we can build. Let's make something fast!
📜 License

This project is licensed under the MIT License. See the LICENSE file for details.
