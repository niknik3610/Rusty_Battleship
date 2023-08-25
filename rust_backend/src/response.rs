use std::{net::TcpStream, io::{Write, Read}, fs::File};
use anyhow::anyhow;
use crate::{router::{self, RouterError}, battleship_game::{Game, Board}};
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
pub fn handle_get_request(req: Request, con: TcpStream, game: &Game) {
    let split_uri: Vec<&str> = req.uri.url.split("/").collect();

    match split_uri[0] {
        "update_board" => {
            let requesting_player = split_uri[1].parse().unwrap(); //TODO: Possibly change
            updated_board_req(requesting_player, con, game);
        } 
        _ => response_file(con, &req.uri.url[..]),
    }
}
fn updated_board_req(requesting_player_id: usize, mut con: TcpStream, game: &Game) {
    let board: anyhow::Result<&Board>;
    if game.current_turn == requesting_player_id {
        board = game.get_board_attack(requesting_player_id);
    }
    else {
        board = game.get_board_priv(requesting_player_id);
    }
    
    match board {
        Ok(r) => {
            println!("Updated Player {}'s board", requesting_player_id);
            let board_json = serde_json::to_string(r).unwrap();
            con.write_all(board_json.as_bytes()).unwrap();
        },
        Err(e) => {
            eprintln!("{}", e.to_string());
            response_404(con);
        }
    }
}

enum PostRequestType {
    RegisterClient,
    AliveSquare,
    KillSquare,
    RequestClientID,
}
pub fn handle_post_request(req: Request, mut con: TcpStream, game: &mut Game) {
    let split_uri: Vec<&str> = req.uri.url.split("/").collect();

    match split_uri[0] {
        "request_client_id" => {
            let c_id = game.player_connection();
            let response = serde_json::to_string(&c_id).unwrap();
            con.write_all(response.as_bytes()).unwrap();
        }
        _ => {
            response_404(con)
        }
    }
}
