use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Other error encountered")]
    Other,
}
