use axum::{response::Html, Json, debug_handler};
use serde::{Deserialize, Serialize};
use crate::services::MediaService;
use crate::models::MediaType;

#[derive(Debug, Deserialize)]
pub struct RenameRequest {
    pub new_filename: String,
    pub media_type: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[debug_handler]
pub async fn media_page() -> Html<String> {
    let service = MediaService::new();
    let media = service.get_all_media();

    let mut media_items = String::new();
    for item in &media {
        let type_label = match item.media_type {
            crate::models::MediaType::Photo => "📷 照片",
            crate::models::MediaType::Video => "🎬 视频",
        };

        let media_path = match item.media_type {
            crate::models::MediaType::Photo => format!("/media/photos/{}", item.filename),
            crate::models::MediaType::Video => format!("/media/videos/{}", item.filename),
        };

        let media_element = match item.media_type {
            crate::models::MediaType::Photo => {
                format!(r#"<img src="{}" alt="{}" loading="lazy">"#, media_path, item.filename)
            }
            crate::models::MediaType::Video => {
                format!(r#"<video src="{}" controls preload="metadata"></video>"#, media_path)
            }
        };

        media_items.push_str(&format!(
            r#"<div class="media-item" data-filename="{}" data-type="{}">
                {}
                <div class="media-caption">
                    <span class="media-type">{}</span>
                    <div class="filename">{}</div>
                    <div class="media-actions">
                        <button class="btn-rename" onclick="renameMedia('{}', '{}')">✏️ 重命名</button>
                        <button class="btn-delete" onclick="deleteMedia('{}')">🗑️ 删除</button>
                    </div>
                </div>
            </div>"#,
            item.filename,
            match item.media_type {
                crate::models::MediaType::Photo => "photo",
                crate::models::MediaType::Video => "video",
            },
            media_element, type_label, 
            // 显示时去掉后缀
            {
                let path = std::path::Path::new(&item.filename);
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or(&item.filename)
            },
            // 第一个参数是完整文件名（带后缀），第二个是显示名称（无后缀）
            item.filename,
            {
                let path = std::path::Path::new(&item.filename);
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or(&item.filename)
            },
            item.filename
        ));
    }

    if media_items.is_empty() {
        media_items = String::from(r#"<div style="text-align: center; color: rgba(255,255,255,0.8); padding: 3rem; grid-column: 1 / -1;">
            暂无媒体文件，在 <code style="background: rgba(255,255,255,0.2); padding: 0.2rem 0.5rem; border-radius: 4px;">media/photos/</code> 或 <code style="background: rgba(255,255,255,0.2); padding: 0.2rem 0.5rem; border-radius: 4px;">media/videos/</code> 目录下添加文件即可
        </div>"#);
    }

    let template = include_str!("../../assets/media.html");
    let html = template.replace("{{media_items}}", &media_items);

    Html(html)
}

pub async fn delete_media(
    axum::extract::Path((filename, media_type_str)): axum::extract::Path<(String, String)>,
) -> Json<ApiResponse> {
    let media_type = if media_type_str == "video" {
        MediaType::Video
    } else {
        MediaType::Photo
    };

    let service = MediaService::new();
    match service.delete_media(&filename, media_type) {
        Ok(_) => Json(ApiResponse {
            success: true,
            message: Some("Deleted".to_string()),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            message: None,
            error: Some(e),
        }),
    }
}

pub async fn rename_media(
    axum::extract::Path((filename, media_type_str)): axum::extract::Path<(String, String)>,
    Json(req): Json<RenameRequest>,
) -> Json<ApiResponse> {
    let media_type = if media_type_str == "video" {
        MediaType::Video
    } else {
        MediaType::Photo
    };

    // 自动保留原文件后缀
    let old_ext = std::path::Path::new(&filename)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    
    let new_filename = if old_ext.is_empty() {
        req.new_filename.clone()
    } else {
        format!("{}.{}", req.new_filename, old_ext)
    };

    let service = MediaService::new();
    match service.rename_media(&filename, &new_filename, media_type) {
        Ok(_) => Json(ApiResponse {
            success: true,
            message: Some("Renamed".to_string()),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            message: None,
            error: Some(e),
        }),
    }
}
