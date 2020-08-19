# http-server-from-scratch

## Run

```bash
cargo run
nc 127.0.0.1 8080 <<< 'GET /README.md HTTP/1.1\r\n\r\n'
nc 127.0.0.1 8080 <<< 'GET /src/main.rs HTTP/1.1\r\n\r\n'
```

## References

- [Building a Single-Threaded Web Server - The Rust Programming Language](https://doc.rust-lang.org/book/ch20-01-single-threaded.html)
