use std::{
    net::TcpStream, 
    io::{BufReader, BufRead}, str::FromStr
};

use crate::response::{response_200, response_file};

struct Request {
    pub uri: ParsedUri,

}
impl Request {
    fn new(url: ParsedUri) -> Request {
        return Request {
            uri: url
        }
    }
}

pub fn handle_request(mut con: TcpStream) {
    let buf_reader = BufReader::new(&mut con);
    let stringified_req: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let parsed_req = parse_request(stringified_req).unwrap();
    println!("Url: {:?}", parsed_req.uri);

    match parsed_req.uri.method {
        Method::GET => response_file(con, &parsed_req.uri.url[..])
    }
}

fn parse_request(req: Vec<String>) -> Result<Request, String> {
    let url = req[0].clone();
    return Ok(Request::new(
        parse_url(url).unwrap()
    ));
}

#[derive(Debug)]
struct ParsedUri {
    method: Method,
    url: String,
    version: String
}

#[derive(Debug)]
enum Method {
    GET,
}
impl FromStr for Method {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => return Ok(Method::GET),
            _ => return Err(())
        }
    }
}

fn parse_url(url: String) -> Result<ParsedUri, String> {
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
