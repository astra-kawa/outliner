use super::{InterfaceError, NodeStore};
use crate::domain::Node;

pub struct SqliteStore {}

impl NodeStore for SqliteStore {
    fn create_node(text: &str) -> Result<Node, InterfaceError> {
        let node_result = Node::new(text);

        Ok(node_result.unwrap())
    }
}
