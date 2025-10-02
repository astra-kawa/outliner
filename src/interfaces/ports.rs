use super::errors::InterfaceError;
use crate::domain::{
    Node,
    models::{NodeType, Source},
};
use uuid::Uuid;

pub trait NodeRepository {
    fn create_node(
        &self,
        parent: Option<Uuid>,
        previous: Option<Uuid>,
        node_type: NodeType,
        text: &str,
        author: &str,
        source: Source,
    ) -> Result<Node, InterfaceError>;

    fn get_node(&self, node_id: &Uuid) -> Result<Node, InterfaceError>;

    fn update_node(&self, node: &Node) -> Result<(), InterfaceError>;

    fn dump_nodes(&self) -> Result<Vec<Node>, InterfaceError>;
}
