use super::errors::InterfaceError;
use crate::domain::Node;
use uuid::Uuid;

pub trait NodeRepository {
    fn add_node(&self, node: &Node) -> Result<(), InterfaceError>;

    fn get_node(&self, node_id: &Uuid) -> Result<Node, InterfaceError>;

    fn update_node(&self, node: &Node) -> Result<(), InterfaceError>;

    fn delete_node(&self, node_id: &Uuid) -> Result<(), InterfaceError>;

    fn dump_nodes(&self) -> Result<Vec<Node>, InterfaceError>;
}
