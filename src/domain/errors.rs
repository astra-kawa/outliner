use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid datetime string provided")]
    InvalidDateTime,
    #[error("Other error encountered")]
    Other,
}
