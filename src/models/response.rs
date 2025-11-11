use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}
