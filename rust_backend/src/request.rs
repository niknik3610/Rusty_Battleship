use std::{
    net::TcpStream, 
    io::{BufReader, BufRead}
};

use crate::response::{response_200, response_file};

struct Request {
    pub url: String
}
impl Request {
    fn new(url: String) -> Request {
        return Request {
            url
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
    println!("Url: {}", parsed_req.url);

    response_file(con, "/");
}

fn parse_request(req: Vec<String>) -> Result<Request, String> {
    let url = req[0].clone();
    return Ok(Request::new(
        url
    ));
}
