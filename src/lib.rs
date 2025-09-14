// src/lib.rs
use pyo3::prelude::*;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

mod thread_pool;
use thread_pool::ThreadPool;

/// One HTTP handler: responds with “Hello, <name>!” JSON.
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer); // ignore errors for brevity
                                      // very naive parsing: expect "GET /hello/<name> HTTP/1.1"
    let request = String::from_utf8_lossy(&buffer);
    let name = request
        .split_whitespace()
        .nth(1)
        .unwrap_or("/")
        .trim_start_matches("/hello/")
        .split('/')
        .next()
        .unwrap_or("world");
    let body = format!(r#"{{"message":"Hello, {}!"}}"#, name);
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}

/// Launch the server on `0.0.0.0:port` (port 0 ⇒ OS-chosen) in background threads.
/// Returns the chosen port so Python knows where to connect.
#[pyfunction]
fn start_server(thread_count: usize, port: Option<u16>) -> PyResult<u16> {
    let bind_addr = format!("0.0.0.0:{}", port.unwrap_or(0));
    let listener = TcpListener::bind(&bind_addr)?;
    let actual_port = listener.local_addr()?.port();

    // move listener into a detached thread so Python call returns immediately
    std::thread::spawn(move || {
        let pool = ThreadPool::new(thread_count.max(1));
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                pool.execute(|| handle_connection(stream));
            }
        }
    });
    Ok(actual_port)
}

/// PyO3 module definition
#[pymodule]
fn mini_server(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_server, m)?)?;
    Ok(())
}
