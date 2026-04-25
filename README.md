Multi-threaded Rust Web Server (From Scratch)
Welcome! I'm Vesper.rs, a Computer Engineering student at Nakhon Phanom University. This project was created as a learning exercise to deeply understand how a web server operates in Rust without relying on high-level runtime frameworks like tokio.

Project Overview
The goal of this project is to build a functional HTTP web server from the ground up, focusing on low-level systems programming concepts. By implementing a manual thread pool and handling TCP connections directly, I gained hands-on experience with Rust's ownership, concurrency, and memory safety model.

Key Features
TCP Listener: Binds to 127.0.0.1:7878 to accept incoming client connections.

Custom Thread Pool: A manual implementation of a thread pool using std::thread and mpsc (Multi-producer, single-consumer) channels to manage concurrent tasks efficiently.

Concurrency Management: Uses Arc<Mutex<Receiver<Job>>> to safely share the task receiver across multiple worker threads.

Graceful Shutdown: Implements the Drop trait for the ThreadPool to ensure all threads are properly joined and cleaned up when the server stops.

Request Handling: Basic HTTP/1.1 request parsing for GET and POST methods, including a simulated slow response at /sleep to test multi-threaded performance.

Technical Stack
Language: Rust (Edition 2024)

Libraries: Standard library only (std::net, std::thread, std::sync)—no external async runtimes.

Learning Goals
Understanding how TcpListener and TcpStream work at a system level.

Implementing the Worker-Pool pattern for efficient resource management.

Mastering thread safety in Rust using Mutex and Arc.

Handling manual memory and thread cleanup using the Drop trait.
