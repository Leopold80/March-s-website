mod server;
mod handlers;
mod services;
mod models;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    server::start().await;
}
