use std::net::TcpListener;
use crate::{
    SERVER_ADDR,
    request::handle_request,
    battleship_game::Game
};

pub async fn run_server() {
    let mut game = Game::new();
    
    let listener = TcpListener::bind(SERVER_ADDR)
        .unwrap();

    listener.incoming()
        .for_each(|request|{
            let request = request.unwrap();
            handle_request(request, &mut game);
        })
}
