use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 统一的 API 响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl ApiResponse {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: Some(message.into()),
            error: None,
            data: None,
        }
    }

    pub fn success_with_data(message: impl Into<String>, data: Value) -> Self {
        Self {
            success: true,
            message: Some(message.into()),
            error: None,
            data: Some(data),
        }
    }

    pub fn error(error: impl Into<String>) -> Self {
        Self {
            success: false,
            message: None,
            error: Some(error.into()),
            data: None,
        }
    }
}

/// 上传限制 (30GB)
pub const UPLOAD_MAX_SIZE: usize = 30 * 1024 * 1024 * 1024;
