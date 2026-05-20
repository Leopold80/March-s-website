use axum::{response::Html, Json, debug_handler, body::Body, http::{StatusCode, header}};
use axum::response::Response;
use bytes::Bytes;
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

        let view_link = format!("/view/{}/{}",
            match item.media_type {
                MediaType::Photo => "photo",
                MediaType::Video => "video",
            },
            item.filename
        );

        let media_element = match item.media_type {
            MediaType::Photo => {
                let thumb_path = format!("/thumbnail/{}", item.filename);
                format!(r#"<a href="{}"><img src="{}" alt="{}" loading="lazy"></a>"#, view_link, thumb_path, item.filename)
            }
            MediaType::Video => {
                let poster_path = format!("/video-poster/{}", item.filename);
                let has_compressed = if item.compressed_filename.is_some() { "true" } else { "false" };
                let compressed_filename = item.compressed_filename.clone().unwrap_or_default();
                format!(r#"<div class="video-container" data-filename="{}" data-has-compressed="{}" data-compressed="{}">
                    <a href="/view/video/{}"><img src="{}" alt="{}" loading="lazy"></a>
                </div>"#,
                    item.filename, has_compressed, compressed_filename,
                    item.filename, poster_path, item.filename)
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

    let new_filename = crate::utils::file_extension(&filename)
        .map(|ext| format!("{}.{}", req.new_filename, ext))
        .unwrap_or(req.new_filename);

    let service = MediaService::new();
    match service.rename_media(&filename, &new_filename, media_type) {
        Ok(_) => Json(ApiResponse::success("Renamed")),
        Err(e) => Json(ApiResponse::error(e)),
    }
}

pub async fn get_thumbnail(axum::extract::Path(filename): axum::extract::Path<String>) -> Result<Response<Body>, (StatusCode, String)> {
    let service = MediaService::new();
    let source_path = service.get_photo_path(&filename);

    if !source_path.exists() {
        return Err((StatusCode::NOT_FOUND, "File not found".to_string()));
    }

    let _thumb_filename = match service.generate_thumbnail(&source_path, &filename) {
        Ok(name) => name,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    };

    let thumb_path = service.get_thumbnail_path(&filename);

    match tokio::fs::read(&thumb_path).await {
        Ok(data) => {
            let mut response = Response::new(Body::from(Bytes::from(data)));
            response.headers_mut().insert(header::CONTENT_TYPE, "image/jpeg".parse().unwrap());
            response.headers_mut().insert(header::CACHE_CONTROL, "public, max-age=31536000".parse().unwrap());
            Ok(response)
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_video_poster(axum::extract::Path(filename): axum::extract::Path<String>) -> Result<Response<Body>, (StatusCode, String)> {
    let service = MediaService::new();
    let source_path = service.get_video_path(&filename);

    if !source_path.exists() {
        return Err((StatusCode::NOT_FOUND, "File not found".to_string()));
    }

    let _poster_filename = match service.generate_video_poster(&source_path, &filename) {
        Ok(name) => name,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    };

    let poster_path = service.get_video_poster_path(&filename);

    match tokio::fs::read(&poster_path).await {
        Ok(data) => {
            let mut response = Response::new(Body::from(Bytes::from(data)));
            response.headers_mut().insert(header::CONTENT_TYPE, "image/jpeg".parse().unwrap());
            response.headers_mut().insert(header::CACHE_CONTROL, "public, max-age=31536000".parse().unwrap());
            Ok(response)
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[debug_handler]
pub async fn view_media_page(
    axum::extract::Path((media_type, filename)): axum::extract::Path<(String, String)>,
) -> Html<String> {
    let template = include_str!("../../assets/view_media.html");

    let media_url = match media_type.as_str() {
        "photo" => format!("/media/photos/{}", filename),
        "video" => format!("/media/videos/{}", filename),
        _ => return Html("Invalid media type".to_string()),
    };

    let media_content = match media_type.as_str() {
        "photo" => format!(r#"<div class="media-wrapper"><img src="{}" alt="{}"></div>"#, media_url, filename),
        "video" => format!(r#"<div class="media-wrapper"><video src="{}" controls preload="metadata" poster="/video-poster/{}?t={}"></video></div>"#, media_url, filename, chrono::Utc::now().timestamp()),
        _ => String::new(),
    };

    let html = template
        .replace("{{media_url}}", &media_url)
        .replace("{{filename}}", &filename)
        .replace("{{media_type}}", &match media_type.as_str() {
            "photo" => "📷 照片",
            "video" => "🎬 视频",
            _ => "媒体",
        })
        .replace("{{media_content}}", &media_content);

    Html(html)
}
