# Blaze Webserver

An experimental, high-performance web server built in Rust, focusing on speed, memory safety, and low-level control.

<details>
<summary><strong>Table of Contents</strong></summary>
<ol>
<li><a href="#-about">About</a></li>
<li><a href="#-features">Features</a></li>
<li><a href="#-getting-started">Getting Started</a></li>
<li><a href="#-roadmap">Roadmap</a></li>
<li><a href="#-license">License</a></li>
</ol>
</details>

## 📖 About

🚧 **Status:** Blaze is experimental and not yet ready for production.

This project prioritizes a deep, foundational understanding of web server mechanics through a minimalist, performance-oriented design.

  * ⚡ **Performance First:** Built for high throughput, low latency, and a minimal memory footprint.
  * 🛡️ **Memory Safe:** Uses Rust's ownership model to eliminate common vulnerabilities like buffer overflows at compile time.
  * ⚙️ **Minimalist:** Built from scratch to avoid unnecessary abstractions and provide granular control over the request-response lifecycle.

## ✨ Features

  * 🌐 **Async Core:** Built on `tokio` for a non-blocking, event-driven architecture that handles thousands of concurrent connections efficiently.
  * 📜 **HTTP/1.1 Parser:** Custom implementation for handling raw TCP streams and translating them into structured requests.
  * 📁 **Static File Serving:** Serves static assets (HTML, CSS, JS) from a designated public directory.

## 🚀 Getting Started

### Prerequisites

  * **Rust Toolchain:** [https://rustup.rs/](https://rustup.rs/)

### Build & Run

1.  Clone the repository:
    ```sh
    git clone https://github.com/chro11os/blaze-webserver.git
    cd blaze-webserver
    ```
2.  Run the server:
    ```sh
    cargo run
    ```

By default, the server starts on `127.0.0.1:8080`.

## 🗺️ Roadmap

  - [ ] **Core Features**
      - [x] Basic TCP Listener
      - [ ] Robust HTTP/1.1 Request Parsing
      - [ ] Response Serialization
      - [ ] Concurrency using a Thread Pool
  - [ ] **Advanced Features**
      - [ ] Basic Routing
      - [ ] Middleware Support
      - [ ] Configuration from file (`.toml`)
      - [ ] TLS/HTTPS Support
      - [ ] Logging
  - [ ] **Performance**
      - [ ] Benchmarking
      - [ ] Optimizations

## 📜 License

This project is licensed under the **MIT License**. See the `LICENSE` file for details.
