use crate::{
    domain::{Node, node::CreateNodeRequest},
    interfaces::NodeRepository,
    services::{errors::ServiceError, logging::LoggingService},
};

pub trait NodeService {
    fn create_node(&self, request: CreateNodeRequest) -> Result<Node, ServiceError>;

    fn update_node(&self, node: &mut Node, new_text: &str) -> Result<(), ServiceError>;

    fn delete_node(&self, node: Node) -> Result<(), ServiceError>;
}

pub struct Service<R, L>
where
    R: NodeRepository,
    L: LoggingService,
{
    pub repository: R,
    pub logger: L,
}

impl<R, L> NodeService for Service<R, L>
where
    R: NodeRepository,
    L: LoggingService,
{
    fn create_node(&self, request: CreateNodeRequest) -> Result<Node, ServiceError> {
        let node = Node::new(request).map_err(ServiceError::Domain)?;

        self.repository
            .add_node(&node)
            .map_err(ServiceError::Interface)?;

        Ok(node)
    }

    fn update_node(&self, node: &mut Node, new_text: &str) -> Result<(), ServiceError> {
        node.update(new_text).map_err(ServiceError::Domain)?;

        self.repository
            .update_node(node)
            .map_err(ServiceError::Interface)?;

        Ok(())
    }

    fn delete_node(&self, node: Node) -> Result<(), ServiceError> {
        // todo: figure out how to handle deleting a node when it contains children
        // additionally, if deleted node has a sibling, update the next node's previous_id
        self.repository
            .delete_node(&node.id())
            .map_err(ServiceError::Interface)?;

        Ok(())
    }
}
