use crate::models::LogMeta;
use std::fs;
use std::path::PathBuf;

pub struct LogService {
    logs_dir: PathBuf,
}

impl LogService {
    pub fn new() -> Self {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_| ".".to_string());
        Self {
            logs_dir: PathBuf::from(manifest_dir).join("md_notes"),
        }
    }

    pub fn create_log(&self, slug: &str, title: &str, date: &str, content: &str) -> Result<(), String> {
        let log_path = self.logs_dir.join(format!("{}.md", slug));
        if log_path.exists() {
            return Err("Log already exists".to_string());
        }
        self.write_log_file(&log_path, title, date, content)
    }

    pub fn update_log(&self, slug: &str, title: &str, date: &str, content: &str) -> Result<(), String> {
        let log_path = self.logs_dir.join(format!("{}.md", slug));
        if !log_path.exists() {
            return Err("Log not found".to_string());
        }
        self.write_log_file(&log_path, title, date, content)
    }

    pub fn delete_log(&self, slug: &str) -> Result<(), String> {
        let log_path = self.logs_dir.join(format!("{}.md", slug));
        if log_path.exists() {
            fs::remove_file(&log_path).map_err(|e| format!("Failed to delete: {}", e))?;
        }
        Ok(())
    }

    fn write_log_file(&self, path: &PathBuf, title: &str, date: &str, content: &str) -> Result<(), String> {
        let frontmatter = format!("---\ntitle: {}\ndate: {}\n---\n\n", title, date);
        let full_content = format!("{}{}", frontmatter, content);
        fs::create_dir_all(self.logs_dir.parent().unwrap()).ok();
        fs::write(path, full_content).map_err(|e| format!("Failed to write: {}", e))
    }

    pub fn get_all_logs(&self) -> Vec<LogMeta> {
        let mut logs = Vec::new();

        if !self.logs_dir.exists() {
            return logs;
        }

        if let Ok(entries) = fs::read_dir(&self.logs_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                        // 从文件修改时间获取日期
                        let date = Self::get_file_date(&path);
                        
                        // 标题：从文件名提取，去掉日期前缀（如果有）
                        let title = Self::extract_title_from_filename(filename);
                        
                        logs.push(LogMeta {
                            slug: filename.to_string(),
                            title,
                            date,
                        });
                    }
                }
            }
        }

        logs.sort_by(|a, b| b.date.cmp(&a.date));
        logs
    }

    /// 从文件修改时间获取日期字符串
    fn get_file_date(path: &PathBuf) -> String {
        fs::metadata(path)
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| {
                let datetime: chrono::DateTime<chrono::Local> = t.into();
                Some(datetime.format("%Y-%m-%d").to_string())
            })
            .unwrap_or_else(|| "未知日期".to_string())
    }

    /// 从文件名提取标题，去掉日期前缀（如 2024-01-15-）
    fn extract_title_from_filename(filename: &str) -> String {
        // 尝试去掉 YYYY-MM-DD- 前缀
        let title = if filename.len() > 11 && filename.chars().nth(4) == Some('-') && filename.chars().nth(7) == Some('-') {
            &filename[11..] // 跳过 "YYYY-MM-DD-"
        } else {
            filename
        };
        
        // 将连字符替换为空格，使标题更易读
        title.replace('-', " ")
    }

    pub fn get_log_content(&self, slug: &str) -> Option<(String, String, String)> {
        let log_path = self.logs_dir.join(format!("{}.md", slug));

        if !log_path.exists() {
            return None;
        }

        let content = fs::read_to_string(&log_path).ok()?;
        
        // 尝试解析 frontmatter（为了向后兼容）
        let (title, date, markdown_body) = if content.starts_with("---") {
            // 有 frontmatter，解析它
            let (title, date) = Self::parse_frontmatter(&content)?;
            let body_start = content.find("---").and_then(|first| {
                content[first + 3..].find("---").map(|second| first + second + 6)
            }).unwrap_or(0);
            let markdown_body = content[body_start..].to_string();
            (title, date, markdown_body)
        } else {
            // 没有 frontmatter，从文件名和文件时间获取
            let title = Self::extract_title_from_filename(slug);
            let date = Self::get_file_date(&log_path);
            (title, date, content)
        };

        Some((title, date, markdown_body))
    }

    fn parse_frontmatter(content: &str) -> Option<(String, String)> {
        if !content.starts_with("---") {
            return None;
        }

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

        Some((
            title.unwrap_or_else(|| "无标题".to_string()),
            date.unwrap_or_else(|| "未知日期".to_string()),
        ))
    }
}
