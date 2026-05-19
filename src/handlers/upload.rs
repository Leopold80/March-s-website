use axum::{extract::Multipart, response::Html, Json};
use serde::{Deserialize, Serialize};
use crate::services::LogService;
use crate::models::MediaType;
use tokio::io::AsyncWriteExt;
use futures_util::{StreamExt, TryStreamExt};

pub async fn write_log_page() -> Html<String> {
    let template = include_str!("../../assets/write_log.html");
    Html(template.to_string())
}

pub async fn upload_media_page() -> Html<String> {
    let template = include_str!("../../assets/upload_media.html");
    Html(template.to_string())
}

pub async fn upload_error_page() -> Html<String> {
    let template = include_str!("../../assets/error.html");
    let html = template.replace("{{error_message}}", "文件大小超过 30GB 限制，请压缩后重新上传");
    Html(html)
}

#[derive(Debug, Deserialize)]
pub struct CreateLogRequest {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLogRequest {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub async fn upload_media(mut multipart: Multipart) -> Json<ApiResponse> {
    let mut filename: Option<String> = None;
    let mut media_type_str = String::from("photo");
    let mut file_size: u64 = 0;

    while let Some(field) = multipart.next_field().await.ok().flatten() {
        let name = field.name().unwrap_or("");
        if name == "file" {
            let name = field.file_name().unwrap_or("unknown").to_string();
            filename = Some(name.clone());

            let mut file = match tokio::fs::File::create(&name).await {
                Ok(f) => f,
                Err(e) => {
                    return Json(ApiResponse {
                        success: false,
                        message: None,
                        error: Some(format!("Failed to create file: {}", e)),
                    });
                }
            };

            // Use axum's Multipart field directly - it's already a stream
            let mut stream = field.into_stream();
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        file_size += bytes.len() as u64;
                        if let Err(e) = file.write_all(&bytes).await {
                            return Json(ApiResponse {
                                success: false,
                                message: None,
                                error: Some(format!("Failed to write: {}", e)),
                            });
                        }
                    }
                    Err(e) => {
                        return Json(ApiResponse {
                            success: false,
                            message: None,
                            error: Some(format!("Stream error: {}", e)),
                        });
                    }
                }
            }
            let _ = file.flush().await;
        } else if name == "type" {
            if let Ok(text) = field.text().await {
                media_type_str = text;
            }
        }
    }

    let Some(filename) = filename else {
        return Json(ApiResponse {
            success: false,
            message: None,
            error: Some("No file uploaded".to_string()),
        });
    };

    if file_size == 0 {
        let _ = tokio::fs::remove_file(&filename).await;
        return Json(ApiResponse {
            success: false,
            message: None,
            error: Some("Uploaded file is empty".to_string()),
        });
    }

    let media_type = match media_type_str.as_str() {
        "video" => MediaType::Video,
        _ => MediaType::Photo,
    };

    let subdir = match media_type {
        MediaType::Photo => "photos",
        MediaType::Video => "videos",
    };

    let target_dir = std::path::PathBuf::from("media").join(subdir);
    if let Err(e) = tokio::fs::create_dir_all(&target_dir).await {
        let _ = tokio::fs::remove_file(&filename).await;
        return Json(ApiResponse {
            success: false,
            message: None,
            error: Some(format!("Failed to create directory: {}", e)),
        });
    }

    let target_path = target_dir.join(&filename);
    if let Err(e) = tokio::fs::rename(&filename, &target_path).await {
        let _ = tokio::fs::remove_file(&filename).await;
        return Json(ApiResponse {
            success: false,
            message: None,
            error: Some(format!("Failed to move file: {}", e)),
        });
    }

    Json(ApiResponse {
        success: true,
        message: Some(format!("Uploaded: {}", filename)),
        error: None,
    })
}

pub async fn create_log(Json(req): Json<CreateLogRequest>) -> Json<ApiResponse> {
    let service = LogService::new();
    match service.create_log(&req.slug, &req.title, &req.date, &req.content) {
        Ok(_) => Json(ApiResponse {
            success: true,
            message: Some("Log created".to_string()),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            message: None,
            error: Some(e),
        }),
    }
}

pub async fn update_log(Json(req): Json<UpdateLogRequest>) -> Json<ApiResponse> {
    let service = LogService::new();
    match service.update_log(&req.slug, &req.title, &req.date, &req.content) {
        Ok(_) => Json(ApiResponse {
            success: true,
            message: Some("Log updated".to_string()),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            message: None,
            error: Some(e),
        }),
    }
}

pub async fn delete_log(slug: String) -> Json<ApiResponse> {
    let service = LogService::new();
    match service.delete_log(&slug) {
        Ok(_) => Json(ApiResponse {
            success: true,
            message: Some("Log deleted".to_string()),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            message: None,
            error: Some(e),
        }),
    }
}
