mod router;
mod server;
mod request;
mod response;
mod battleship_game;
mod api_structs;

use std::sync::Mutex;
use lazy_static::lazy_static;

pub const SERVER_ADDR: &str = "127.0.0.1:8000";

lazy_static! {
    pub static ref ARGS: Mutex<CmdArgs> = Mutex::new(
        CmdArgs {
            cached: false,
        });
}

#[derive(Debug)]
pub struct CmdArgs {
    pub cached: bool,
}

#[tokio::main]
async fn main() {
    gather_args();
    println!("Using Caching: {:?}", ARGS.lock().unwrap().cached);
    server::run_server().await;  
}

fn gather_args() {
    let args: Vec<String> = std::env::args().collect();
    let parsed_args: Vec<Args> = args.iter()
        .map(|arg| match_arg(arg))
        .filter(|arg| {
            if let Some(_) = arg {
                return true
            }
            return false
        })
        .map(|arg| arg.unwrap())
        .collect(); 

    let mut args = ARGS.lock().unwrap();
    parsed_args.iter().for_each(|arg|{
        match arg {
            Args::Cached => args.cached = true
        }
    });
}

enum Args {
    Cached,
}
fn match_arg(arg: &str) -> Option<Args> {
    match arg {
        "-cached" | "-c" => return Some(Args::Cached),
        _ => None
    }
}
