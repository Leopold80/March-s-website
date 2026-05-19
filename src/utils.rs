use std::path::Path;

/// 从文件名中获取不带后缀的名称
pub fn filename_without_ext(filename: &str) -> &str {
    Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(filename)
}

/// 获取文件扩展名（小写）
pub fn file_extension(filename: &str) -> Option<String> {
    Path::new(filename)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
}
