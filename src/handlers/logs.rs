use axum::{response::Html, debug_handler, extract::Path};
use pulldown_cmark::{Parser, Options, html};
use crate::services::LogService;

#[debug_handler]
pub async fn logs_page() -> Html<String> {
    let service = LogService::new();
    let logs = service.get_all_logs();

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

    let template = include_str!("../../assets/logs.html");
    let html = template.replace("{{log_items}}", &log_items);

    Html(html)
}

#[debug_handler]
pub async fn log_post_page(Path(slug): Path<String>) -> Html<String> {
    let service = LogService::new();

    let (title, date, markdown_body) = match service.get_log_content(&slug) {
        Some(content) => content,
        None => return Html(not_found_page(&slug)),
    };

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(&markdown_body, options);

    let mut html_body = String::new();
    html::push_html(&mut html_body, parser);

    let template = include_str!("../../assets/log_post.html");
    let html = template
        .replace("{{title}}", &title)
        .replace("{{date}}", &format!("📅 {}", date))
        .replace("{{content}}", &html_body);

    Html(html)
}

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
