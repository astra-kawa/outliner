use crate::{
    domain::{Node, node::CreateNodeRequest},
    interfaces::NodeRepository,
    services::errors::ServiceError,
};

// pub trait NodeService {
//     fn create_node(&self, request: CreateNodeRequest) -> Result<Node, ServiceError>;
//     fn update_node(&self, node: &mut Node, new_text: &str) -> Result<(), ServiceError>;
//     fn delete_node(&self, node: Node) -> Result<(), ServiceError>;
// }

pub struct NodeService<R>
where
    R: NodeRepository,
{
    pub repository: R,
}

impl<R> NodeService<R>
where
    R: NodeRepository,
{
    pub fn create_node(&self, request: CreateNodeRequest) -> Result<Node, ServiceError> {
        let node = Node::new(request).map_err(ServiceError::Domain)?;

        self.repository
            .add_node(&node)
            .map_err(ServiceError::Interface)?;

        Ok(node)
    }

    pub fn update_node(&self, node: &mut Node, new_text: &str) -> Result<(), ServiceError> {
        node.update(new_text).map_err(ServiceError::Domain)?;

        self.repository
            .update_node(node)
            .map_err(ServiceError::Interface)?;

        Ok(())
    }

    pub fn delete_node(&self, node: Node) -> Result<(), ServiceError> {
        // todo: figure out how to handle deleting a node when it contains children
        // additionally, if deleted node has a sibling, update the next node's previous_id
        self.repository
            .delete_node(&node.id())
            .map_err(ServiceError::Interface)?;

        Ok(())
    }
}
