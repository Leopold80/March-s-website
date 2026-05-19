use axum::{response::Html, debug_handler};

#[debug_handler]
pub async fn hello_page() -> Html<&'static str> {
    Html(include_str!("../../assets/index.html"))
}
