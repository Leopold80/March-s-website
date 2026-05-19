use axum::{response::Html, debug_handler};
use crate::services::MediaService;

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
            r#"<div class="media-item">
                {}
                <div class="media-caption">
                    <span class="media-type">{}</span>
                    <div>{}</div>
                </div>
            </div>"#,
            media_element, type_label, item.filename
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
