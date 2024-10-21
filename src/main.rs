// Prelude: This program should be simple TCP/HTTP server
// + abstraction layer to easily create req/res routes
// i.e. something like Node + Express
// P.S. I am learning Rust through doing this so bear with me :D

use std::{io::{Write, Read}, net::TcpStream, thread, env, fs};
use std::net::TcpListener;

struct Env {
    dirname: String,
}

struct Request {
    query: String,
    env: Env,
    headers: Vec<String>,
}

type RequestHandler = fn(stream: TcpStream, req: Request);

struct Route {
    path: String,
    handler: RequestHandler,
    matcher: String,
}

struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn create_route(&mut self, path: String, matcher: String, handler: RequestHandler) {
        let route = Route {
            path,
            handler,
            matcher,
        };

        self.routes.push(route);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let mut handles = vec![];

    for stream in listener.incoming() {
        let handle = thread::spawn(move || {
            let args: Vec<String> = env::args().collect();
            match stream {
                Ok(mut _stream) => {
                    // Chop and extract <parts> separately

                    // <URL Extractor>
                    let mut buf = [0; 512];
                    let _ = _stream.read(&mut buf);
                    let request = String::from_utf8(buf.to_vec()).unwrap();
                    let path_start = request.find("GET ").unwrap() + "GET ".len();
                    let path_end = request.find(" HTTP").unwrap();
                    let resource = &request[path_start..path_end];

                    // <Headers>
                    // 1. Extract different parts
                    let pattern = "HTTP/1.1\r\n";
                    let mut headers_pointer = request.find(pattern).unwrap() + pattern.len();
                    let mut headers: Vec<String> = Vec::new();
                    while let Some(header_length) = &request[headers_pointer..].find("\r\n") {
                        let header_end = headers_pointer + *header_length;
                        let header = &request[headers_pointer..header_end];
                        if header == "" {
                            break;
                        }

                        headers.push(header.to_string());
                        headers_pointer = header_end + "\r\n".len();
                    }

                    // 2. How to serve different parts
                    // 3. Router - Matched - give req, res header


                    // <Server> = <Resource URL> -> <Server Response>
                    // too abstract but will do for now
                    declare_and_execute_server(resource, args, headers, _stream);
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }

        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    fn declare_and_execute_server(resource: &str, args: Vec<String>, headers: Vec<String>, stream: TcpStream) {
        // do both sides of the abstraction layer here
        // 1. declare the server - declare routes, handlers etc
        let mut routes = Vec::<Route>::new();

        // TODO: Expand the functionality later
        // fn format_response(status, headers, body) -> ()
        fn format_response_with_body(body: &str) -> String {
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);

            return response;
        }

        fn format_file_response(body: String) -> String {
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);

            return response;
        }

        fn get_header_value(headers: Vec<String>, header_name: &str) -> String {
            let header_string = headers.iter().find(|header| header.contains(&header_name)).unwrap();
            let value_start = header_string.find(&header_name).unwrap() + header_name.len() + ": ".len();
            let body = &header_string[value_start..];

            return body.to_string();
        }

        fn get_query(resource: &str, matcher: &str) -> String {
            if let Some(match_start) = resource.find(matcher) {
                let match_end = match_start + matcher.len();
                // Avoid buffer overflow
                if match_end < resource.len() {
                    let query_start = match_end + "/".len();
                    return (&resource[query_start..]).to_string();
                }

                return "".to_string();
            }
                
            return "".to_string()
        }

        fn get_env(args: Vec<String>) -> Env {
            let dirname = args[2].clone();

            return Env {
                dirname
            }
        }

        // TODO: Add interface functionality to allow optional arguments
        fn handle_root(mut stream: TcpStream, _: Request) {
            let _ = stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes());
        }

        fn handle_echo(mut stream: TcpStream, req: Request) {
            // Echo query back
            let response = format_response_with_body(&req.query);
            let _ = stream.write_all(response.as_bytes());
        }

        fn handle_files(mut stream: TcpStream, req: Request) {
            let Env { dirname } = req.env;
            let file_name = req.query;
            let file_path = format!("{}{}", dirname, file_name);
            let contents = fs::read_to_string(file_path);

            match contents {
                Ok(content) => {
                    let response = format_file_response(content);
                    let _ = stream.write_all(response.as_bytes());
                }
                Err(_) => {
                    handle_404(stream.try_clone().unwrap(), "");
                }
            }
        }

        fn handle_user_agent(mut stream: TcpStream, req: Request) {
            // TODO: Enhance and extract the header parser

            let body = get_header_value(req.headers, "User-Agent");
            let response = format_response_with_body(&body);
            let _ = stream.write_all(response.as_bytes());
        }

        fn handle_404(mut stream: TcpStream, _: &str) {
            let _ = stream.write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes());
        }

        let root_handler: RequestHandler = handle_root;
        let echo_handler: RequestHandler = handle_echo;
        let user_agent_handler: RequestHandler = handle_user_agent;
        let files_handler: RequestHandler = handle_files;

        let root = Route {
            path: "/".to_string(),
            matcher: "/".to_string(),
            handler: root_handler,
        };

        let echo = Route {
            path: "/echo/{query}".to_string(),
            matcher: "/echo".to_string(),
            handler: echo_handler,
        };

        let files = Route {
            path: "/files/{query}".to_string(),
            matcher: "/files".to_string(),
            handler: files_handler,
        };

        let user_agent = Route {
            path: "/user-agent".to_string(),
            matcher: "/user-agent".to_string(),
            handler: user_agent_handler,
        };

        routes.push(files);
        routes.push(echo);
        routes.push(user_agent);
        routes.push(root);

        // 2. Glue the generated server to low level request "choppers"
        // by router
        // TODO: Implement a route matcher using regex
        // TODO: Extract the <Parser> from the <Router>
        let match_route = routes.iter().find(|route| {
            if resource == "/" {
                return route.matcher == "/";
            }

            return route.matcher != "/" && resource.contains(&route.matcher);
        });

        if let Some(route) = match_route {
            // TODO: Write request details parser
            let request = Request {
                query: get_query(resource, &route.matcher),
                headers: headers.clone(),
                env: get_env(args),
            };
            // TODO: Try to fix this magic
            (route.handler)(stream.try_clone().unwrap(), request);
        } else {
            // Error 404
            handle_404(stream.try_clone().unwrap(), "");
        }
    }    
}
