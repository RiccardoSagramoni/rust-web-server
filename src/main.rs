use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

use rust_web_server::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::build(4).unwrap();
    
    for stream in listener.incoming().take(2) {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    let _ = handle_connection(stream);
                })
                .unwrap();
            }
            Err(err) => println!(
                "TcpListener::incoming() failed with the following error: {err}"
            ),
        };
    }
}


fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    // Read the request line
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .ok_or::<Box<dyn Error>>("no request line".into())??;
    
    // Select the status line to return as response and the name of the file
    // with the response content, depending from the user's request
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "html/hello.html"),
        "GET /sleep HTTP/1.1" => {
            // Simulate a slow request
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "html/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "html/404.html"),
    };
    
    // Build the response
    let contents = fs::read_to_string(filename)?;
    let length = contents.len();
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    // Write the response to the stream
    stream.write_all(response.as_bytes())?;
    Ok(())
}
