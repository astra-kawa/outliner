use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterfaceError {
    #[error("Table creation error")]
    TableCreation,
    #[error("Node creation error")]
    NodeCreation,
    #[error("Node write error")]
    NodeWrite,
    #[error("Node update error")]
    NodeUpdate,
    #[error("Other error encountered")]
    Other,
}
