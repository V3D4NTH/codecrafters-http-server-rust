use std::io::{
    Read,
    Write
};
use std::net::TcpListener;
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buffer = [0; 512];
                stream.read(&mut buffer).unwrap();

                let request = String::from_utf8_lossy(&buffer[..]);
                let headers = request.lines();
                let request_line = request.lines().next().unwrap();
                let url_path = request_line.split_whitespace().nth(1).unwrap();
                if url_path == "/user-agent" {
                    let mut user_agent = "";

                    for header in headers {
                        if header.to_lowercase().starts_with("user-agent:") {
                            user_agent = header.split(":").nth(1).unwrap().trim();
                            break;
                        }
                    }


                    let content_length = user_agent.len();
                    let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                            content_length, user_agent
                    );

                    let _ = stream.write_all(response.as_bytes()).unwrap();
                } else if url_path.starts_with("/echo/") {
                    let response_str = &url_path[6..];
                    let content_length = response_str.len();
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                        content_length, response_str
                    );
                    stream.write_all(response.as_bytes()).unwrap();
                } else {
                    let response = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
                    let _ = stream.write_all(response.as_bytes()).unwrap();
                }

            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}