use std::fs;
use std::io::prelude::*;
use std::io::Cursor;
use std::net::{TcpListener, TcpStream};

fn handle_header(buffer: &[u8]) -> Result<(String, String), ()> {
    let mut cursor = Cursor::new(buffer);
    let mut buf = String::new();
    let _ = cursor.read_line(&mut buf);
    let mut header = buf.split_whitespace();
    let method: String = header.next().unwrap().into();
    let path: String = header.next().unwrap().into();

    Ok((method, path))
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (status_line, filename): (&str, String) = if let Ok((method, path)) = handle_header(&buffer)
    {
        println!("{} {}", method, path);
        ("HTTP/1.1 200 OK\r\n\r\n", path[1..].to_owned())
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "README.md".to_owned())
    };

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => return,
    };
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                panic!("encountered IO error: {}", e);
            }
        }
    }
    Ok(())
}
