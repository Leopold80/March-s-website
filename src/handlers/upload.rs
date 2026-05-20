use axum::{extract::Multipart, response::Html, Json};
use serde::Deserialize;
use crate::services::{LogService, MediaService};
use crate::models::MediaType;
use crate::types::ApiResponse;
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
    let html = template.replace(
        "{{error_message}}",
        &format!("文件大小超过 {}GB 限制，请压缩后重新上传", crate::types::UPLOAD_MAX_SIZE / 1024 / 1024 / 1024)
    );
    Html(html)
}

#[derive(Debug, Deserialize)]
pub struct CreateLogRequest {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub content: String,
}

pub async fn upload_media(mut multipart: Multipart) -> Json<ApiResponse> {
    let mut filename: Option<String> = None;
    let mut media_type_str = String::from("photo");
    let mut file_data: Vec<u8> = Vec::new();

    while let Some(field) = multipart.next_field().await.ok().flatten() {
        let name = field.name().unwrap_or("");
        if name == "file" {
            let name = field.file_name().unwrap_or("unknown").to_string();
            filename = Some(name.clone());

            let mut stream = field.into_stream();
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        file_data.extend_from_slice(&bytes);
                    }
                    Err(e) => {
                        return Json(ApiResponse::error(format!("Stream error: {}", e)));
                    }
                }
            }
        } else if name == "type" {
            if let Ok(text) = field.text().await {
                media_type_str = text;
            }
        }
    }

    let Some(filename) = filename else {
        return Json(ApiResponse::error("No file uploaded"));
    };

    if file_data.is_empty() {
        return Json(ApiResponse::error("Uploaded file is empty"));
    }

    let media_type = match media_type_str.as_str() {
        "video" => MediaType::Video,
        _ => MediaType::Photo,
    };

    let service = MediaService::new();
    match service.upload_media(&filename, media_type, &file_data) {
        Ok(final_filename) => Json(ApiResponse::success(format!("Uploaded: {}", final_filename))),
        Err(e) => Json(ApiResponse::error(e)),
    }
}

pub async fn create_log(Json(req): Json<CreateLogRequest>) -> Json<ApiResponse> {
    let service = LogService::new();
    match service.create_log(&req.slug, &req.title, &req.date, &req.content) {
        Ok(_) => Json(ApiResponse::success("Log created")),
        Err(e) => Json(ApiResponse::error(e)),
    }
}
