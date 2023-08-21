use std::{net::TcpStream, io::{Write, Read}, fs::File};
use anyhow::anyhow;
use crate::router::{self, RouterError};
use crate::request::Request;

pub fn response_200(mut con: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    con.write_all(response.as_bytes()).unwrap();
}

fn response_404(mut con: TcpStream) {
    let response = "HTTP/1.1 404 Not Found\r\n\r\n";
    con.write_all(response.as_bytes()).unwrap();
}

fn response_500(mut con: TcpStream) {
    let response = "HTTP/1.1 500 Internal Server Error\r\n\r\n";
    con.write_all(response.as_bytes()).unwrap();
}

pub fn response_file(mut con: TcpStream, url: &str) {
    println!("{url}");

    let status_line = "HTTP/1.1 200 OK";
    let file_contents_result = router::route(url);
    let file_contents: String;

    match file_contents_result {
        Ok(r) =>  file_contents = r,
        Err(e) => {
            match_router_error(e, con);
            return;
        }
    }

    let content_length = file_contents.len();
    let content_type = "text/html; charset=UTF-8";

    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\nContent-Type: {content_type}\r\n\r\n{file_contents}");
    con.write_all(response.as_bytes()).unwrap();
}

fn get_file(path: String) -> anyhow::Result<String> {
    let file_result = File::open(path); 
    let mut file: File;

    match file_result {
        Ok(r) => file = r,
        Err(_) => return Err(anyhow!("Could Not find File"))
    }

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    return Ok(contents)
}

fn match_router_error(e: RouterError, con: TcpStream) {
    match e.error_type {
        router::RouterErrorType::Type404 => response_404(con),
        _ => response_500(con)
    }
}

enum GetRequestType {
    SiteRequest,
    BoardRequest,
}
fn handle_get_request(req: Request, con: TcpStream) {
    let split_uri: Vec<&str> = req.uri.url.split("/").collect();

    match split_uri[0] {
        "update_board" => todo!(),
        _ => response_file(con, &req.uri.url[..]),
    }
}
fn update_board(player: u32, con: &mut TcpStream) {
    //TODO
    con.write_all("Nothing Here Rn".as_bytes());
}

enum PutRequestType {
    RegisterClient,
    AliveSquare,
    KillSquare,
}
fn handle_put_request(uri: &str) {

}
