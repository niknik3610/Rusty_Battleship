use std::{
    net::TcpStream, 
    io::{BufReader, BufRead}, str::FromStr, any
};

use anyhow::anyhow;
use crate::{response::{response_200, response_file, handle_get_request, handle_post_request}, battleship_game::Game};

pub struct Request {
    pub uri: ParsedUri,
}
impl Request {
    fn new(url: ParsedUri) -> Request {
        return Request {
            uri: url
        }
    }
}

pub fn handle_request(mut con: TcpStream, game: &mut Game) {
    let buf_reader = BufReader::new(&mut con);
    let stringified_req: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let parsed_req = parse_request(stringified_req).unwrap();
    match parsed_req.uri.method {
        Method::GET => handle_get_request(parsed_req, con, game),
        Method::POST => handle_post_request(parsed_req, con, game),
    }
}

fn parse_request(req: Vec<String>) -> anyhow::Result<Request> {
    let url = req[0].clone();
    return Ok(Request::new(
        parse_url(url).unwrap()
    ));
}

#[derive(Debug)]
pub struct ParsedUri {
    method: Method,
    pub url: String,
    version: String
}

#[derive(Debug)]
enum Method {
    GET,
    POST,
}
impl Method {
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            _ => Err(anyhow!("Unknown HTTP Request Method"))
        }
    }
}

fn parse_url(url: String) -> anyhow::Result<ParsedUri, String> {
    let mut curr_field = 0;
    let mut fields = vec![
        String::new(),      //Method
        String::new(),      //Url
        String::new()       //Version
    ];

    url.chars().for_each(|char|{
        if char == ' ' {
            curr_field += 1;
        }
        else {

        }
    });

    let parsed_url = ParsedUri {
        method: Method::from_str(&fields[0][..]).unwrap(),
        url: fields[1].clone(),
        version: fields[2].clone()
    };

    return Ok(parsed_url);
}
