// ============================================================================
// 服务器配置模块
// ============================================================================

use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use crate::pages;

/// 创建应用路由配置
pub fn create_app() -> Router {
    Router::new()
        .route("/", get(pages::hello_page))         // 首页路由
        .route("/logs", get(pages::logs_page))      // 日志列表页
        .route("/log/{slug}", get(pages::log_post_page))  // 单篇日志阅读页
        .route("/media", get(pages::media_page))    // 媒体墙页面
        .nest_service("/media/photos", ServeDir::new("media/photos"))  // 照片静态服务
        .nest_service("/media/videos", ServeDir::new("media/videos"))  // 视频静态服务
        // 可以添加更多路由，例如：
        // .route("/about", get(about_page))      // 关于页面
        // .route("/api/status", get(api_status)) // API 接口
}

/// 启动服务器
pub async fn start() {
    pages::init_media();

    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("Server running at http://0.0.0.0:3000");

    axum::serve(listener, app).await.expect("Server failed");
}
