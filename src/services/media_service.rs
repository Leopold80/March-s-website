use crate::models::{MediaItem, MediaType};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub struct MediaService {
    media_dir: PathBuf,
    cache_dir: PathBuf,
    ffmpeg_available: bool,
}

impl MediaService {
    pub fn new() -> Self {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_| ".".to_string());
        let media_dir = PathBuf::from(&manifest_dir).join("media");
        let cache_dir = media_dir.join("cache");
        let ffmpeg_available = Command::new("ffmpeg")
            .arg("-version")
            .output()
            .is_ok();
        
        if !cache_dir.exists() {
            let _ = fs::create_dir_all(&cache_dir);
        }
        
        Self {
            media_dir,
            cache_dir,
            ffmpeg_available,
        }
    }

    pub fn upload_media(&self, filename: &str, media_type: MediaType, data: &[u8]) -> Result<String, String> {
        let subdir = match media_type {
            MediaType::Photo => "photos",
            MediaType::Video => "videos",
        };

        let target_dir = self.media_dir.join(subdir);
        fs::create_dir_all(&target_dir).map_err(|e| format!("Failed to create directory: {}", e))?;

        let target_path = target_dir.join(filename);
        let mut file = fs::File::create(&target_path).map_err(|e| format!("Failed to create file: {}", e))?;
        file.write_all(data).map_err(|e| format!("Failed to write file: {}", e))?;

        if media_type == MediaType::Photo && filename.to_lowercase().ends_with(".heic") && self.ffmpeg_available {
            let jpg_path = target_path.with_extension("jpg");
            let output = Command::new("ffmpeg")
                .arg("-i")
                .arg(&target_path)
                .arg("-y")
                .arg(&jpg_path)
                .output();
            if output.is_ok() {
                let _ = fs::remove_file(&target_path);
                return Ok(jpg_path.file_name().unwrap().to_str().unwrap().to_string());
            }
        }

        Ok(filename.to_string())
    }

    pub fn delete_media(&self, filename: &str, media_type: MediaType) -> Result<(), String> {
        let subdir = match media_type {
            MediaType::Photo => "photos",
            MediaType::Video => "videos",
        };
        
        let file_path = self.media_dir.join(subdir).join(filename);
        if file_path.exists() {
            fs::remove_file(&file_path).map_err(|e| format!("Failed to delete: {}", e))?;
        }
        Ok(())
    }

    pub fn rename_media(&self, filename: &str, new_filename: &str, media_type: MediaType) -> Result<(), String> {
        let subdir = match media_type {
            MediaType::Photo => "photos",
            MediaType::Video => "videos",
        };
        
        let old_path = self.media_dir.join(subdir).join(filename);
        let new_path = self.media_dir.join(subdir).join(new_filename);
        
        if !old_path.exists() {
            return Err(format!("File not found: {}", filename));
        }
        
        if new_path.exists() {
            return Err(format!("File already exists: {}", new_filename));
        }
        
        fs::rename(&old_path, &new_path).map_err(|e| format!("Failed to rename: {}", e))?;
        Ok(())
    }

    pub fn get_all_media(&self) -> Vec<MediaItem> {
        let mut items = Vec::new();

        if !self.media_dir.exists() {
            return items;
        }

        let photo_exts = ["jpg", "jpeg", "png", "gif", "webp"];
        let video_exts = ["mp4", "webm", "mov"];

        let photos_dir = self.media_dir.join("photos");
        if photos_dir.exists() {
            if self.ffmpeg_available {
                self.scan_heic_files(&photos_dir);
            }

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

        let videos_dir = self.media_dir.join("videos");
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

    fn scan_heic_files(&self, photos_dir: &PathBuf) {
        if let Ok(entries) = fs::read_dir(photos_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()).map(|e| e.to_lowercase()) == Some("heic".to_string()) {
                    self.convert_heic_to_jpg(&path);
                }
            }
        }
    }

    fn convert_heic_to_jpg(&self, heic_path: &PathBuf) {
        let jpg_path = heic_path.with_extension("jpg");
        let output = Command::new("ffmpeg")
            .arg("-i")
            .arg(heic_path)
            .arg("-y")
            .arg(&jpg_path)
            .output();
        if output.is_ok() {
            let _ = fs::remove_file(heic_path);
        }
    }

    pub fn get_video_poster_path(&self, filename: &str) -> PathBuf {
        self.cache_dir.join(format!("poster_{}.jpg", filename))
    }

    pub fn generate_video_poster(&self, video_path: &PathBuf, filename: &str) -> Result<String, String> {
        let poster_path = self.get_video_poster_path(filename);

        if poster_path.exists() {
            return Ok(format!("poster_{}.jpg", filename));
        }

        let output = Command::new("ffmpeg")
            .arg("-i")
            .arg(video_path)
            .arg("-vf")
            .arg("thumbnail")
            .arg("-frames:v")
            .arg("1")
            .arg("-y")
            .arg(&poster_path)
            .output()
            .map_err(|e| format!("Failed to extract poster: {}", e))?;

        if output.status.success() {
            Ok(format!("poster_{}.jpg", filename))
        } else {
            Err("Failed to extract video poster".to_string())
        }
    }

    pub fn get_thumbnail_path(&self, filename: &str) -> PathBuf {
        self.cache_dir.join(format!("thumb_{}", filename))
    }

    pub fn generate_thumbnail(&self, source_path: &PathBuf, filename: &str) -> Result<String, String> {
        let thumb_path = self.get_thumbnail_path(filename);
        
        if thumb_path.exists() {
            return Ok(format!("thumb_{}", filename));
        }

        let output = Command::new("ffmpeg")
            .arg("-i")
            .arg(source_path)
            .arg("-vf")
            .arg("scale=800:-1")
            .arg("-q:v")
            .arg("85")
            .arg("-y")
            .arg(&thumb_path)
            .output()
            .map_err(|e| format!("Failed to generate thumbnail: {}", e))?;
        
        if output.status.success() {
            Ok(format!("thumb_{}", filename))
        } else {
            Err("Failed to generate thumbnail".to_string())
        }
    }

    pub fn get_photo_path(&self, filename: &str) -> PathBuf {
        self.media_dir.join("photos").join(filename)
    }

    pub fn get_video_path(&self, filename: &str) -> PathBuf {
        self.media_dir.join("videos").join(filename)
    }
}
