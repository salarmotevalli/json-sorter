pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Clone)]
pub enum AppError {
    Json(String),
    File(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::Json(e) => write!(f, "json: {}", e),
            AppError::File(e) => write!(f, "file: {}", e),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> AppError {
        AppError::Json(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::File(err.to_string())
    }
}
