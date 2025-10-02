use thiserror::Error;

use crate::domain::DomainError;

#[derive(Error, Debug)]
pub enum InterfaceError {
    #[error("Failed to connect to DB")]
    DbConnection,
    #[error("Table creation error")]
    TableCreation,
    #[error("Invalid query error")]
    InvalidQuery,
    #[error("Node creation error")]
    NodeCreation,
    #[error("Node write error")]
    NodeWrite,
    #[error("Node update error")]
    NodeUpdate,
    #[error("Operation performed on missing node")]
    MissingNodeOperation,
    #[error("Node was not found in DB")]
    MissingNode,
    #[error("Error when parsing field: `{0}`")]
    FieldParseError(String),
    #[error("Domain Error: `{0}`")]
    Domain(DomainError),
    #[error("Other error encountered")]
    Other,
}
