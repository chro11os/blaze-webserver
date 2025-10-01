# Blaze Webserver

## An experimental, high-performance web server built from scratch in Rust, focusing on speed, memory safety, and a low-level understanding of web protocols.

<details>
<summary><strong>Table of Contents</strong></summary>
<ol>
<li><a href="#-project-status">Project Status</a></li>
<li><a href="#-core-philosophy">Core Philosophy</a></li>
<li><a href="#-architecture-and-features">Architecture and Features</a></li>
<li><a href="#-getting-started">Getting Started</a></li>
<li><a href="#-project-roadmap">Roadmap</a></li>
<li><a href="#-contributing">Contributing</a></li>
<li><a href="#-license">License</a></li>
</ol>
</details>

## 🚧 Project Status
### Blaze is currently under active development and is intended for educational and experimental purposes. It is not yet ready for production use.

## 💡 Core Philosophy
### This project prioritizes a deep, foundational understanding of web server mechanics.

## ⚡ Performance First
### The primary goal is raw speed. Every architectural decision is made with the aim of achieving a high requests-per-second count, low latency, and a minimal memory footprint.

## 🛡️ Security Through Memory Safety
### By leveraging Rust's ownership model and strict compiler, Blaze aims to eliminate entire classes of common security vulnerabilities, such as buffer overflows, and data races, at compile time.

## ⚙️ Minimalism and Control
### Blaze is being built from first principles to avoid unnecessary abstractions and dependencies. This provides granular control over the entire request-response lifecycle and serves as a clear example of systems programming.

## 🛠️ Architecture and Features
### 🌐 Asynchronous TCP Core
### The server is built on tokio, a powerful asynchronous runtime for Rust. This allows for a non-blocking, event-driven architecture capable of handling thousands of concurrent connections with high efficiency.

## 📜 HTTP/1.1 Implementation
## A custom parser is being developed to handle raw TCP streams and translate them into structured HTTP/1.1 requests. The focus is on a compliant and performant implementation of the protocol's core features.

## 📁 Static Asset Serving
### The server includes a basic file system handler for serving static assets like HTML, CSS, and JavaScript files directly from a designated public directory.

## 🚀 Getting Started

## Prerequisites

    Rust Toolchain (latest stable version recommended): https://rustup.rs/

## Build and Run

    Clone the repository:

    git clone [https://github.com/your-username/blaze-webserver.git](https://github.com/your-username/blaze-webserver.git)
    cd blaze-webserver

    Run in development mode:

    cargo run

By default, the server will start on 127.0.0.1:8080.

## 🗺️ Project Roadmap

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

## 📜 License

## This project is licensed under the MIT License. See the LICENSE file for details.
