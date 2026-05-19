// ============================================================================
// 页面处理模块
// ============================================================================

use axum::{response::Html, debug_handler, extract::Path};
use pulldown_cmark::{Parser, Options, html};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[debug_handler]
pub async fn hello_page() -> Html<&'static str> {
    Html(include_str!("../assets/index.html"))
}

// ============================================================================
// 扩展示例
// ============================================================================

// 关于页面示例（取消注释即可使用）
// #[debug_handler]
// pub async fn about_page() -> Html<&'static str> {
//     Html(include_str!("../assets/about.html"))
// }

// API 接口示例
// use axum::Json;
// use serde::Serialize;
//
// #[derive(Serialize)]
// struct Status {
//     system: String,
//     uptime: u64,
// }
//
// #[debug_handler]
// pub async fn api_status() -> Json<Status> {
//     Json(Status {
//         system: "running".to_string(),
//         uptime: 42,
//     })
// }

// ============================================================================
// 日志功能
// ============================================================================

#[derive(Debug, Clone)]
pub struct LogMeta {
    pub slug: String,
    pub title: String,
    pub date: String,
}

fn get_logs_dir() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(manifest_dir).join("logs")
}

fn parse_frontmatter(content: &str) -> Option<(String, String)> {
    if !content.starts_with("---") {
        return None;
    }

    // 查找 frontmatter 结束位置
    let end = content[3..].find("---")? + 3;
    let frontmatter = &content[3..end];

    let mut title = None;
    let mut date = None;

    for line in frontmatter.lines() {
        let line = line.trim();
        if let Some((key, value)) = line.split_once(':') {
            match key.trim() {
                "title" => title = Some(value.trim().to_string()),
                "date" => date = Some(value.trim().to_string()),
                _ => {}
            }
        }
    }

    Some((title.unwrap_or_else(|| "无标题".to_string()),
          date.unwrap_or_else(|| "未知日期".to_string())))
}

fn get_all_logs() -> Vec<LogMeta> {
    let logs_dir = get_logs_dir();
    let mut logs = Vec::new();

    if !logs_dir.exists() {
        return logs;
    }

    if let Ok(entries) = fs::read_dir(&logs_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Some((title, date)) = parse_frontmatter(&content) {
                            logs.push(LogMeta {
                                slug: filename.to_string(),
                                title,
                                date,
                            });
                        }
                    }
                }
            }
        }
    }

    logs.sort_by(|a, b| b.date.cmp(&a.date));
    logs
}

#[debug_handler]
pub async fn logs_page() -> Html<String> {
    let logs = get_all_logs();

    let mut log_items = String::new();
    for log in &logs {
        log_items.push_str(&format!(
            r#"<li class="log-item">
                <a href="/log/{}">
                    <div class="log-title">{}</div>
                    <div class="log-date">{}</div>
                </a>
            </li>"#,
            log.slug, log.title, log.date
        ));
    }

    if log_items.is_empty() {
        log_items = String::from(r#"<li style="text-align: center; color: rgba(255,255,255,0.8); padding: 2rem;">
            暂无日志，在 <code style="background: rgba(255,255,255,0.2); padding: 0.2rem 0.5rem; border-radius: 4px;">logs/</code> 目录下创建 Markdown 文件即可
        </li>"#);
    }

    let template = include_str!("../assets/logs.html");
    let html = template.replace("{{log_items}}", &log_items);

    Html(html)
}

#[debug_handler]
pub async fn log_post_page(Path(slug): Path<String>) -> Html<String> {
    let logs_dir = get_logs_dir();
    let log_path = logs_dir.join(format!("{}.md", slug));

    if !log_path.exists() {
        return Html(not_found_page(&slug));
    }

    let content = match fs::read_to_string(&log_path) {
        Ok(c) => c,
        Err(_) => return Html(not_found_page(&slug)),
    };

    let (title, date) = parse_frontmatter(&content).unwrap_or_else(|| {
        ("无标题".to_string(), "未知日期".to_string())
    });

    let body_start = content.find("---").and_then(|first| {
        content[first + 3..].find("---").map(|second| first + second + 6)
    }).unwrap_or(0);

    let markdown_body = &content[body_start..];

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(markdown_body, options);

    let mut html_body = String::new();
    html::push_html(&mut html_body, parser);

    let template = include_str!("../assets/log_post.html");
    let html = template
        .replace("{{title}}", &title)
        .replace("{{date}}", &format!("📅 {}", date))
        .replace("{{content}}", &html_body);

    Html(html)
}

/// 404 页面
fn not_found_page(slug: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 - 未找到</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            min-height: 100vh;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            display: flex;
            align-items: center;
            justify-content: center;
            text-align: center;
            color: white;
        }}
        h1 {{ font-size: 4rem; margin-bottom: 1rem; }}
        p {{ font-size: 1.2rem; margin-bottom: 2rem; opacity: 0.9; }}
        a {{ color: white; text-decoration: none; border: 2px solid white; padding: 0.8rem 2rem; border-radius: 8px; }}
        a:hover {{ background: white; color: #667eea; }}
    </style>
</head>
<body>
    <div>
        <h1>404</h1>
        <p>日志 "{}" 不存在</p>
        <a href="/">返回首页</a>
    </div>
</body>
</html>"#,
        slug
    )
}

// ============================================================================
// 媒体墙功能
// ============================================================================

fn convert_heic_to_jpg(heic_path: &PathBuf) -> Option<PathBuf> {
    let jpg_path = heic_path.with_extension("jpg");

    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(heic_path)
        .arg("-y")
        .arg(&jpg_path)
        .output()
        .ok()?
        .status;

    if status.success() {
        Some(jpg_path)
    } else {
        None
    }
}

fn scan_heic_files(photos_dir: &PathBuf) {
    if !photos_dir.exists() {
        return;
    }

    if let Ok(entries) = fs::read_dir(photos_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()).map(|e| e.to_lowercase()) == Some("heic".to_string()) {
                println!("Converting HEIC: {:?}", path);
                if let Some(jpg_path) = convert_heic_to_jpg(&path) {
                    println!("Converted to: {:?}", jpg_path);
                } else {
                    eprintln!("Failed to convert: {:?}", path);
                }
            }
        }
    }
}

pub fn init_media() {
    // 检查 ffmpeg 是否存在
    match Command::new("ffmpeg").arg("-version").output() {
        Ok(_) => println!("ffmpeg detected, HEIC conversion enabled"),
        Err(_) => eprintln!("⚠️  WARNING: ffmpeg not found. HEIC files will not be converted."),
    }
}

#[derive(Debug, Clone)]
pub struct MediaItem {
    pub filename: String,
    pub media_type: MediaType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MediaType {
    Photo,
    Video,
}

fn get_media_dir() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(manifest_dir).join("media")
}

fn get_all_media() -> Vec<MediaItem> {
    let media_dir = get_media_dir();
    let mut items = Vec::new();

    let photo_exts = ["jpg", "jpeg", "png", "gif", "webp"];
    let video_exts = ["mp4", "webm", "mov"];

    if !media_dir.exists() {
        return items;
    }

    let photos_dir = media_dir.join("photos");
    if photos_dir.exists() {
        scan_heic_files(&photos_dir);
        if let Ok(entries) = fs::read_dir(&photos_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    if photo_exts.contains(&ext_lower.as_str()) && ext_lower != "heic" {
                        items.push(MediaItem {
                            filename: path.file_name().unwrap().to_str().unwrap().to_string(),
                            media_type: MediaType::Photo,
                        });
                    }
                }
            }
        }
    }

    let videos_dir = media_dir.join("videos");
    if videos_dir.exists() {
        if let Ok(entries) = fs::read_dir(&videos_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if video_exts.contains(&ext.to_lowercase().as_str()) {
                        items.push(MediaItem {
                            filename: path.file_name().unwrap().to_str().unwrap().to_string(),
                            media_type: MediaType::Video,
                        });
                    }
                }
            }
        }
    }

    items
}

#[debug_handler]
pub async fn media_page() -> Html<String> {
    let media = get_all_media();

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

    let template = include_str!("../assets/media.html");
    let html = template.replace("{{media_items}}", &media_items);

    Html(html)
}
