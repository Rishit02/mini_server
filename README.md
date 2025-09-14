# mini_server

A pure Rust HTTP server that can be started as a Python module using PyO3. It demonstrates building a simple multithreaded web server without external web frameworks and exposing it seamlessly to Python.

---

## Features

- Basic HTTP server handling GET requests using Rust’s `TcpListener` and standard library.
- Multithreaded worker pool for handling concurrent connections efficiently.
- Exposes Rust server functions as a native Python module with PyO3.
- Easy installation and usage using `maturin`.
- Responds with JSON greeting messages on `/hello/<name>` endpoint.
- Designed to emulate a lightweight Flask-like experience with Rust's speed and safety.

---

## Technologies Used

- Rust language with Rust standard library (no external HTTP frameworks).
- PyO3 for Python bindings and creating native Python extension modules.
- Maturin for easy build and packaging of Rust crates as Python wheels.
- Python `requests` for client interaction with the server.

---

## Installation

1. Create and activate a Python virtual environment:

```bash
python -m venv venv
source venv/bin/activate  # Linux/macOS
venv\Scripts\activate     # Windows
```

2. Install `maturin` for building and installing the Rust Python extension:

```bash
pip install maturin
```

3. Build and install the module:

```bash
maturin develop --release
```

---

## Usage

Start the Rust server from Python and send an HTTP request (`pip install requests`):

```python
from mini_server import start_server
import requests
import time

# Start the server with 4 threads and let OS pick the port (port=None)
port = start_server(thread_count=4, port=None)
time.sleep(0.2)  # Wait for server to start

url = f"http://127.0.0.1:{port}/hello/Rustacean"
response = requests.get(url)
print(response.json())  # {'message': 'Hello, Rustacean!'}
```

---

## Project Structure

```
mini_server/
├── Cargo.toml          # Rust crate configuration
├── src/
│   ├── lib.rs          # HTTP server + PyO3 bridge code
│   └── thread_pool.rs  # Thread pool implementation
├── README.md           # This file
```

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

## References

- PyO3: https://pyo3.rs/v0.26.0/
- Rust Book Chapter 21: Building a Multithreaded Web Server
- Maturin: https://github.com/PyO3/maturin
