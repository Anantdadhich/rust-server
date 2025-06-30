use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error<E: ToString>(error: E) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(error.to_string()),
        }
    }
}
