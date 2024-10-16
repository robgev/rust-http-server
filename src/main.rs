use std::io::{Write, Read};
#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let mut buf = [0; 512];
                let _ = _stream.read(&mut buf);
                let request = String::from_utf8(buf.to_vec()).unwrap();
                let path_start = request.find("GET /").unwrap() + "GET /".len();
                let path_end = request.find(" HTTP").unwrap();
                let resource = &request[path_start..path_end];

                if resource == "" {
                    let _ = _stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes());
                } else {
                    let _ = _stream.write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes());
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
