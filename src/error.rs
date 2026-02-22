use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LuminaError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Invalid route mapping: {0}")]
    InvalidRoute(String),

    #[error("Upstream service unreachable: {0}")]
    UpstreamError(#[from] reqwest::Error),

    #[error("Internal Server Error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for LuminaError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            LuminaError::ConfigError(_) | LuminaError::YamlError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error")
            }
            LuminaError::InvalidRoute(msg) => (StatusCode::NOT_FOUND, msg.as_str()),
            LuminaError::UpstreamError(_) => (StatusCode::BAD_GATEWAY, "Upstream error"),
            LuminaError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        // Log the actual error for observability
        tracing::error!("Request failed: {:?}", self);

        let body = Json(json!({
            "error": message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}
