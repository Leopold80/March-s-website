use axum::Json;
use serde::Deserialize;
use crate::services::{MediaService, COMPRESSION_JOB, CompressionProgress};
use crate::types::ApiResponse;
use std::thread;

#[derive(Debug, Deserialize)]
pub struct CompressionQuery {
    pub resolution: u32,
}

pub async fn compress_videos(query: axum::extract::Query<CompressionQuery>) -> Json<ApiResponse> {
    let resolution = query.resolution;
    
    if ![1080, 720, 480, 360].contains(&resolution) {
        return Json(ApiResponse::error("Invalid resolution. Choose 1080, 720, 480, or 360."));
    }

    let service = MediaService::new();
    let videos = service.get_all_videos();

    if videos.is_empty() {
        return Json(ApiResponse::error("No videos found"));
    }

    let total = videos.len();
    let job_id = "compression_job_1".to_string();

    *COMPRESSION_JOB.lock().unwrap() = Some(CompressionProgress {
        total,
        current: 0,
        progress: 0.0,
        error: None,
    });

    thread::spawn(move || {
        let service = MediaService::new();
        for (index, video) in videos.iter().enumerate() {
            let _ = service.compress_video(&video.filename, resolution, total, index + 1);
        }
    });

    Json(ApiResponse::success_with_data("Compression started", serde_json::json!({
        "job_id": job_id
    })))
}

pub async fn get_compress_progress(_path: axum::extract::Path<String>) -> Json<ApiResponse> {
    let progress = COMPRESSION_JOB.lock().unwrap();

    match &*progress {
        Some(p) => {
            if let Some(error) = &p.error {
                Json(ApiResponse::error(error.clone()))
            } else {
                Json(ApiResponse::success_with_data("Progress", serde_json::json!({
                    "total": p.total,
                    "current": p.current,
                    "progress": p.progress
                })))
            }
        }
        None => Json(ApiResponse::error("No active compression job")),
    }
}
