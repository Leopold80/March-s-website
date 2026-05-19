mod server;
mod handlers;
mod services;
mod models;

#[tokio::main]
async fn main() {
    server::start().await;
}
