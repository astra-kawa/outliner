use crate::{
    domain::{Node, models::CreateNodeRequest},
    interfaces::NodeRepository,
    services::errors::ServiceError,
};

pub trait NodeService {
    fn create_node(&self, request: CreateNodeRequest) -> Result<Node, ServiceError>;

    fn update_node(&self, node: &mut Node, new_text: &str) -> Result<(), ServiceError>;

    fn delete_node(&self, node: Node) -> Result<(), ServiceError>;
}

pub struct Service<R>
where
    R: NodeRepository,
{
    pub repository: R,
}

impl<R> NodeService for Service<R>
where
    R: NodeRepository,
{
    fn create_node(&self, request: CreateNodeRequest) -> Result<Node, ServiceError> {
        let node = Node::new(request).map_err(|_| ServiceError::Other)?;

        self.repository
            .add_node(&node)
            .map_err(|_| ServiceError::Other)?;

        Ok(node)
    }

    fn update_node(&self, node: &mut Node, new_text: &str) -> Result<(), ServiceError> {
        node.update(new_text).map_err(|_| ServiceError::Other)?;

        self.repository
            .update_node(node)
            .map_err(|_| ServiceError::Other)?;

        Ok(())
    }

    fn delete_node(&self, node: Node) -> Result<(), ServiceError> {
        self.repository
            .delete_node(&node.id())
            .map_err(|_| ServiceError::Other)?;

        Ok(())
    }
}
