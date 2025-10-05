use thiserror::Error;

use crate::{domain::DomainError, interfaces::InterfaceError};

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Domain error encountered: {0}")]
    Domain(DomainError),
    #[error("Interface error encountered: {0}")]
    Interface(InterfaceError),
    #[error("Other error encountered")]
    Other,
}
