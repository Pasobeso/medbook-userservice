use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub message: Option<String>,
}