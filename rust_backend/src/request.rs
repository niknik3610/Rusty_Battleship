use std::{
    net::TcpStream, 
    io::{BufReader, BufRead, Read}, str::FromStr, any
};

use anyhow::anyhow;
use crate::{response::{response_200, response_file, handle_get_request, handle_post_request}, battleship_game::Game};

pub struct Request {
    pub uri: ParsedUri,
    pub body: Option<String>,
}
impl Request {
    pub fn new(url: ParsedUri) -> Request {
        return Request {
            uri: url,
            body: None
        }
    }
    fn add_body(&mut self, body: &str) {
        self.body = Some(body.to_owned());
    }
}

pub fn handle_request(mut con: TcpStream, game: &mut Game) {
    let parsed_req = parse_request_from_stream(&mut con).unwrap();
    match parsed_req.uri.method {
        Method::GET => handle_get_request(parsed_req, con, game),
        Method::POST => handle_post_request(parsed_req, con, game),
    }
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

fn parse_request_from_stream(stream: &mut TcpStream) -> anyhow::Result<Request> {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut header = String::new();

    loop {
        let line_size = reader.read_line(&mut header).unwrap();

        if line_size < 3 {
            break;
        }
    }

    let split_header: Vec<&str> = header.lines().collect();

    let url = split_header[0].clone();
    let parsed_url = parse_url(url).unwrap();
    let mut parsed_req = Request::new(parsed_url);

    if let Method::GET = parsed_req.uri.method {
        return Ok(parsed_req);
    }

    let mut content_len = 0;

    split_header.iter().for_each(|line| {
        if line.starts_with("content-length") {
            let line_split = line.split(":");
            line_split.for_each(|s| {
                if !s.starts_with("content-length") {
                    content_len = s.trim().parse().unwrap();
                }
            })
        }
    });

    println!("content_len: {content_len}"); 

    if content_len == 0 {
        return Ok(parsed_req);
    }

    let mut content_buffer = vec![0; content_len];
    reader.read_exact(&mut content_buffer).unwrap();
    parsed_req.add_body(std::str::from_utf8(&content_buffer[..]).unwrap());
    return Ok(parsed_req);
}

fn parse_url(url: &str) -> anyhow::Result<ParsedUri, String> {
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
            fields[curr_field].push(char);
        }
    });

    let parsed_url = ParsedUri {
        method: Method::from_str(&fields[0][..]).unwrap(),
        url: fields[1].clone(),
        version: fields[2].clone()
    };

    return Ok(parsed_url);
}
