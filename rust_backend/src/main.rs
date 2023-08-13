use warp::Filter;

#[tokio::main]
async fn main() {
    let message = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(message)
        .run(([0, 0, 0, 0], 8000))
        .await
}
