use axum::{routing::get, Router};
use tower_http::services::ServeDir;

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(crate::handlers::hello_page))
        .route("/logs", get(crate::handlers::logs_page))
        .route("/log/{slug}", get(crate::handlers::log_post_page))
        .route("/media", get(crate::handlers::media_page))
        .nest_service("/media/photos", ServeDir::new("media/photos"))
        .nest_service("/media/videos", ServeDir::new("media/videos"))
}

pub async fn start() {
    let ffmpeg_available = check_ffmpeg();
    if ffmpeg_available {
        println!("ffmpeg detected, HEIC conversion enabled");
    } else {
        eprintln!("⚠️  WARNING: ffmpeg not found. HEIC files will not be converted.");
    }

    let app = create_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    println!("Server running at http://0.0.0.0:3000");
    axum::serve(listener, app).await.expect("Server failed");
}

fn check_ffmpeg() -> bool {
    std::process::Command::new("ffmpeg")
        .arg("-version")
        .output()
        .is_ok()
}
