use std::{io::Read, io::Write,net::TcpListener};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut stream = stream;
                let mut buf = [0;1024];
                stream.read(&mut buf).unwrap();
                stream.write(&mut buf).unwrap();
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
