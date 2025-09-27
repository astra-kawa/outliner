use super::errors::InterfaceError;
use crate::domain::Node;

pub trait NodeStore {
    fn create_node(&self, text: &str) -> Result<Node, InterfaceError>;
}
