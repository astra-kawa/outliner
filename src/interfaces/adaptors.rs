use super::errors::InterfaceError;
use crate::domain::{Node, models::Source};
use uuid::Uuid;

pub trait NodeStore {
    fn create_node(
        &self,
        parent: Option<Uuid>,
        previous: Option<Uuid>,
        text: &str,
        author: &str,
        source: Source,
    ) -> Result<Node, InterfaceError>;

    fn get_node(&self, node_id: &Uuid) -> Result<Node, InterfaceError>;

    fn update_node(&self, node: &Node) -> Result<(), InterfaceError>;

    fn dump_nodes(&self) -> Result<Vec<Node>, InterfaceError>;
}
