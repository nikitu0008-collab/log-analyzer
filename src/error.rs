use thiserror::Error;

#[derive(Error, Debug)]
pub enum LogError {
    #[error("Ошибка чтения файла: {0}")]
    Io(#[from] std::io::Error),

    #[error("Ошибка парсинга JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Ошибка парсинга Nginx-лога: {0}")]
    NginxParse(String),

    #[error("Некорректный формат времени: {0}")]
    InvalidTimestamp(String),
}

pub type Result<T> = std::result::Result<T, LogError>;
