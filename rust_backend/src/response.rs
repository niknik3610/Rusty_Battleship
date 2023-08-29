use std::{net::TcpStream, io::{Write, Read}, fs::File};
use anyhow::anyhow;
use serde::Serializer;
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

fn response_201(mut con: TcpStream, body: Option<&str>) {
    let response = "HTTP/1.1 201 Created\r\n";
    if let None = body {    
        let response_without_body = format!("{response}\r\n");
        con.write_all(response_without_body.as_bytes()).unwrap();
        return;
    }
    let response_with_body = format!("{response}Content-type:application/json\r\n\r\n{}", body.unwrap());
    con.write_all(response_with_body.as_bytes()).unwrap();
}

pub fn response_file(mut con: TcpStream, url: &str) {

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

    println!("{}", split_uri[1]);
    match split_uri[1] {
        "updateBoard" => {
            let requesting_player_string = split_uri[2];  
            let singleton_id = get_singleton_resource_id(requesting_player_string);

            if let Some(singleton_id) = singleton_id {
                println!("{singleton_id} requested their board");
                updated_board_req(singleton_id, con, game);
            }
            else {
                response_404(con);
            }
        } 
        _ => {
            println!("Request not found, defauling to file");
            response_file(con, &req.uri.url[..]);
        }
    }
}
fn updated_board_req(requesting_player_id: usize, con: TcpStream, game: &Game) {
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
            let serialized_board =  serde_json::to_string(&r).unwrap();
            response_201(con, Some(&serialized_board[..]));
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
pub fn handle_post_request(req: Request, con: TcpStream, game: &mut Game) {
    match &req.uri.url[..] {
        "/request_client_id" => {
            let c_id = game.player_connection();
            let serialized_id = serde_json::to_string(&c_id).unwrap();
            let response = format!("{{
\"c_id\": {serialized_id}
            }}");
            response_201(con, Some(response.as_str()));
        }
        _ => {
            println!("Unknown Request");
            response_404(con)
        }
    }
}

fn get_singleton_resource_id(url: &str) -> Option<usize> {  
    let url = urlencoding::decode(url);
    if let Err(_) = url {
        return None;
    }

    let mut final_string = String::new();
    let mut within_pars = false;
    let mut done = false;

    url.unwrap()
        .chars()
        .into_iter()
        .for_each(|char|{
            if done {
                return;
            }
            else if char == '}' {
                done = true;
            }
            else if within_pars {
                final_string.push(char);
            }
            else if char == '{' {
                within_pars = true;
            } 
        });

    if final_string.is_empty() {
        return None
    }

    let parsed_string = final_string.trim().parse();

    if let Err(_e) = parsed_string {
        return None
    }
    return Some(parsed_string.unwrap())
}
