use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Other error encountered")]
    Other,
}
