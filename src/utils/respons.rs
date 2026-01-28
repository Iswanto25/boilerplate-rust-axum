use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub struct AppResponse;

impl AppResponse {
    pub fn success<T>(data: T, message: &str, code: StatusCode) -> Response
    where
        T: Serialize + Send + Sync + 'static, // Generic T
    {
        let response_body = ApiResponse { status: code.as_u16(), message: message.to_string(), data: Some(data), error: None };

        let log_msg = message.to_string();
        let log_status = code.as_u16();

        tokio::spawn(async move {
            tracing::info!(source = "AppResponse::success", status = log_status, message = log_msg, "✅ Response Sent");
        });

        (code, Json(response_body)).into_response()
    }

    pub fn error(error_detail: impl ToString, message: &str, code: StatusCode) -> Response {
        let error_string = error_detail.to_string();

        let response_body = ApiResponse::<()> { status: code.as_u16(), message: message.to_string(), data: None, error: Some(error_string.clone()) };

        let log_msg = message.to_string();
        let log_status = code.as_u16();

        tokio::spawn(async move {
            tracing::error!(source = "AppResponse::error", status = log_status, message = log_msg, error = error_string, "❌ Error Response Sent");
        });

        (code, Json(response_body)).into_response()
    }
}
