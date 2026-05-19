use axum::{response::Html, Json};
use axum_extra::extract::Multipart as AxumMultipart;
use serde::{Deserialize, Serialize};
use crate::services::{MediaService, LogService};
use crate::models::MediaType;

pub async fn upload_page() -> Html<String> {
    let template = include_str!("../../assets/upload.html");
    Html(template.to_string())
}

use axum::extract::Query;

#[derive(Debug, Deserialize)]
pub struct EditLogQuery {
    pub slug: Option<String>,
}

pub async fn edit_log_page(Query(query): Query<EditLogQuery>) -> Html<String> {
    let mut template = include_str!("../../assets/edit_log.html").to_string();
    
    if let Some(slug) = &query.slug {
        let service = LogService::new();
        if let Some((title, date, content)) = service.get_log_content(slug) {
            template = template.replace("{{slug}}", slug);
            template = template.replace("{{title}}", &title);
            template = template.replace("{{date}}", &date);
            template = template.replace("{{content}}", &content);
            return Html(template);
        }
    }
    
    template = template.replace("{{slug}}", "");
    template = template.replace("{{title}}", "");
    template = template.replace("{{date}}", &chrono::Local::now().format("%Y-%m-%d").to_string());
    template = template.replace("{{content}}", "");
    Html(template)
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

pub async fn upload_media(mut multipart: AxumMultipart) -> Json<ApiResponse> {
    let mut file_data: Option<(String, Vec<u8>)> = None;
    let mut media_type_str = String::from("photo");
    
    while let Some(field) = multipart.next_field().await.ok().flatten() {
        let name = field.name().unwrap_or("");
        if name == "file" {
            let filename = field.file_name().unwrap_or("unknown").to_string();
            let data = field.bytes().await.unwrap_or_default().to_vec();
            file_data = Some((filename, data));
        } else if name == "type" {
            if let Ok(text) = field.text().await {
                media_type_str = text;
            }
        }
    }
    
    let Some((filename, data)) = file_data else {
        return Json(ApiResponse {
            success: false,
            message: None,
            error: Some("No file uploaded".to_string()),
        });
    };
    
    let media_type = match media_type_str.as_str() {
        "video" => MediaType::Video,
        _ => MediaType::Photo,
    };
    
    let service = MediaService::new();
    match service.upload_media(&filename, media_type, &data) {
        Ok(saved_name) => Json(ApiResponse {
            success: true,
            message: Some(format!("Uploaded: {}", saved_name)),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            message: None,
            error: Some(e),
        }),
    }
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
