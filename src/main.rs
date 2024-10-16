use std::{io::{Write, Read}, net::TcpStream, fmt::format};
#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                // URL Extractor
                let mut buf = [0; 512];
                let _ = _stream.read(&mut buf);
                let request = String::from_utf8(buf.to_vec()).unwrap();
                let path_start = request.find("GET /").unwrap() + "GET /".len();
                let path_end = request.find(" HTTP").unwrap();
                let resource = &request[path_start..path_end];

                // Server - <Resource URL> -> <Server Response>
                server(resource, _stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    // fn create_route(route: String, handler: Fun) -> Route
    //
    // fn router(routes: Route[], url: String) {
    //  if routes.route matches url {
    //      route.handler();
    //  }
    // }

    fn server(resource: &str, mut stream: TcpStream) {
        if resource == "" {
            let _ = stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes());
        } else if let Some(match_start) = resource.find("echo/") {
            let query_start = match_start + "echo/".len();
            let echo_text = &resource[query_start..];
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", echo_text.len(), echo_text);
            let _ = stream.write_all(response.as_bytes());
        } else {
            let _ = stream.write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes());
        }
    }    
}
