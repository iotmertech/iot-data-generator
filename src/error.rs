use thiserror::Error;

#[derive(Debug, Error)]
pub enum MerError {
    #[error("Config error: {0}")]
    Config(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Template error: {0}")]
    Template(String),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Environment variable '{name}' is not set")]
    MissingEnvVar { name: String },
}

pub type Result<T> = std::result::Result<T, MerError>;
