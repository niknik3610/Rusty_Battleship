use std::net::TcpListener;
use crate::{SERVER_ADDR, request::handle_request};

pub async fn run_server() {
    let listener = TcpListener::bind(SERVER_ADDR)
        .unwrap();

    listener.incoming()
        .for_each(|request|{
            let request = request.unwrap();
            handle_request(request);
        })
}
