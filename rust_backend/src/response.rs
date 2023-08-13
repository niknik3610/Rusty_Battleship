use std::{net::TcpStream, io::Write};

use crate::router;

pub fn response_200(mut con: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    con.write_all(response.as_bytes()).unwrap();
}

pub fn response_file(mut con: TcpStream, url: &str) {
    let status_line = "HTTP/1.1 200 OK";
    let file_contents = router::route(url);
    let content_length = file_contents.len();
    let content_type = "text/html; charset=UTF-8";

    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\nContent-Type: {content_type}\r\n\r\n{file_contents}");
    con.write_all(response.as_bytes()).unwrap();
}
