use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            success: true,
            data: Some(data),
            message: None,
        }
    }

    /// 创建自定义状态码的响应
    #[allow(dead_code)]
    pub fn with_code(code: u16, data: Option<T>, message: Option<String>) -> Self {
        Self {
            code,
            success: code >= 200 && code < 300,
            data,
            message,
        }
    }
}

impl ApiResponse<serde_json::Value> {
    /// 创建错误响应
    pub fn error(message: &str) -> Self {
        Self {
            code: 500,
            success: false,
            data: None,
            message: Some(message.to_string()),
        }
    }
}
