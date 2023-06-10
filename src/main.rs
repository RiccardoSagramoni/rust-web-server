use std::{
    error::Error,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        let _ = handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    // Read the request line
    let buf_reader = BufReader::new(&mut stream);
    let request_line = match buf_reader.lines().next() {
        Some(request_line) => request_line?,
        None => return Err("no request line".into()),
    };
    
    // Select the status line to return as response and the name of the file
    // with the response content, depending from the user's request
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "html/404.html")
    };
    
    // Build the response
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    // Write the response to the stream
    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}
