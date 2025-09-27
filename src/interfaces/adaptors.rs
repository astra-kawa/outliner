use super::errors::InterfaceError;
use crate::domain::Node;

pub trait NodeStore {
    fn create_node(text: &str) -> Result<Node, InterfaceError>;
}
