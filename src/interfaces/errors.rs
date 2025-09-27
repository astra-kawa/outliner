use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterfaceError {
    #[error("Other error encountered")]
    Other,
}
