use axum::{routing::{get, post, put, delete}, Router, extract::DefaultBodyLimit};
use tower_http::services::ServeDir;

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(crate::handlers::hello_page))
        .route("/logs", get(crate::handlers::logs_page))
        .route("/log/{slug}", get(crate::handlers::log_post_page))
        .route("/media", get(crate::handlers::media_page))
        .route("/write-log", get(crate::handlers::write_log_page))
        .route("/edit-log", get(crate::handlers::edit_log_page))
        .route("/edit-media", get(crate::handlers::edit_media_page))
        .route("/upload-media", get(crate::handlers::upload_media_page))
        .route("/upload-error", get(crate::handlers::upload_error_page))
        .route("/view/{media_type}/{filename}", get(crate::handlers::view_media_page))
        .route("/thumbnail/{filename}", get(crate::handlers::get_thumbnail))
        .route("/video-poster/{filename}", get(crate::handlers::get_video_poster))
        .route("/api/upload/media", post(crate::handlers::upload_media).layer(DefaultBodyLimit::max(crate::types::UPLOAD_MAX_SIZE)))
        .route("/api/logs", post(crate::handlers::create_log))
        .route("/api/logs/{slug}", put(crate::handlers::update_log))
        .route("/api/logs/{slug}", delete(crate::handlers::delete_log))
        .route("/api/media/{filename}/{media_type}", delete(crate::handlers::delete_media))
        .route("/api/media/{filename}/{media_type}", put(crate::handlers::rename_media))
        .nest_service("/media/photos", ServeDir::new("media/photos"))
        .nest_service("/media/videos", ServeDir::new("media/videos"))
        .nest_service("/media/cache", ServeDir::new("media/cache"))
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
