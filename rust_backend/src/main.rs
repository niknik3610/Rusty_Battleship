mod router;
mod server;
mod request;
mod response;
use lazy_static::lazy_static;

pub const SERVER_ADDR: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    server::run_server().await;

    
}
