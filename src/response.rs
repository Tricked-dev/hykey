#[repr(transparent)]
pub struct ResponseError(anyhow::Error);

impl From<anyhow::Error> for ResponseError {
    fn from(error: anyhow::Error) -> Self {
        Self(error)
    }
}

impl From<axum::Error> for ResponseError {
    fn from(error: axum::Error) -> Self {
        Self(error.into())
    }
}
impl From<axum::http::Error> for ResponseError {
    fn from(error: axum::http::Error) -> Self {
        Self(error.into())
    }
}

impl From<redis::RedisError> for ResponseError {
    fn from(error: redis::RedisError) -> Self {
        Self(error.into())
    }
}

impl From<hyper::Error> for ResponseError {
    fn from(error: hyper::Error) -> Self {
        Self(error.into())
    }
}

impl axum::response::IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            self.0.to_string(),
        )
            .into_response()
    }
}

pub type ResultResponse<T> = std::result::Result<T, ResponseError>;
