use axum::{response::Html, Json, debug_handler};
use serde::Deserialize;
use crate::services::MediaService;
use crate::models::MediaType;
use crate::types::ApiResponse;
use crate::utils::filename_without_ext;

#[derive(Debug, Deserialize)]
pub struct RenameRequest {
    pub new_filename: String,
}

#[derive(Debug, Deserialize)]
pub struct EditMediaQuery {
    pub file: String,
    #[serde(rename = "type")]
    pub media_type: String,
}

#[debug_handler]
pub async fn edit_media_page(axum::extract::Query(params): axum::extract::Query<EditMediaQuery>) -> Html<String> {
    let template = include_str!("../../assets/edit_media.html");
    let html = template
        .replace("{{filename}}", &params.file)
        .replace("{{display_name}}", filename_without_ext(&params.file))
        .replace("{{media_type}}", &params.media_type);
    Html(html)
}

#[debug_handler]
pub async fn media_page() -> Html<String> {
    let service = MediaService::new();
    let media = service.get_all_media();

    let mut media_items = String::new();
    for item in &media {
        let type_label = match item.media_type {
            MediaType::Photo => "📷 照片",
            MediaType::Video => "🎬 视频",
        };

        let media_path = match item.media_type {
            MediaType::Photo => format!("/media/photos/{}", item.filename),
            MediaType::Video => format!("/media/videos/{}", item.filename),
        };

        let media_element = match item.media_type {
            MediaType::Photo => {
                format!(r#"<img src="{}" alt="{}" loading="lazy">"#, media_path, item.filename)
            }
            MediaType::Video => {
                format!(r#"<video src="{}" controls preload="metadata"></video>"#, media_path)
            }
        };

        let display_name = filename_without_ext(&item.filename);
        let media_type_str = match item.media_type {
            MediaType::Photo => "photo",
            MediaType::Video => "video",
        };

        media_items.push_str(&format!(
            r#"<div class="media-item" data-filename="{}" data-type="{}">
                {}
                <div class="media-caption">
                    <span class="media-type">{}</span>
                    <div class="filename">{}</div>
                    <div class="media-actions">
                        <a href="/edit-media?file={}&type={}" class="btn-edit">✏️ 编辑</a>
                    </div>
                </div>
            </div>"#,
            item.filename,
            media_type_str,
            media_element,
            type_label,
            display_name,
            item.filename,
            media_type_str
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
        Ok(_) => Json(ApiResponse::success("Deleted")),
        Err(e) => Json(ApiResponse::error(e)),
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
    let new_filename = crate::utils::file_extension(&filename)
        .map(|ext| format!("{}.{}", req.new_filename, ext))
        .unwrap_or(req.new_filename);

    let service = MediaService::new();
    match service.rename_media(&filename, &new_filename, media_type) {
        Ok(_) => Json(ApiResponse::success("Renamed")),
        Err(e) => Json(ApiResponse::error(e)),
    }
}
