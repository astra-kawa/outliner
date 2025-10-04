use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid datetime string provided")]
    InvalidDateTime,
    #[error("Failed to parse supplied field: `{0}`")]
    FieldParseError(String),
}
