use serde::Serialize;

/// Application-wide error type. Serialized to the frontend as a structured object
/// (not a raw string) so the UI can render distinct messages per error kind rather
/// than a generic "failed".
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

impl AppError {
    pub fn user_message(&self) -> String {
        match self {
            AppError::Io(io_err) => match io_err.kind() {
                std::io::ErrorKind::StorageFull => "Disk full".to_string(),
                std::io::ErrorKind::PermissionDenied => "Permission denied".to_string(),
                _ => format!("File error: {io_err}"),
            },
            AppError::Other(msg) => msg.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
struct SerializableError {
    message: String,
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SerializableError {
            message: self.user_message(),
        }
        .serialize(serializer)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
