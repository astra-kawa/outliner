use crate::{domain::Node, interfaces::NodeRepository, services::errors::ServiceError};

pub trait NodeService {
    fn update_node(&self, node: &mut Node, new_text: &str) -> Result<(), ServiceError>;
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
    fn update_node(&self, node: &mut Node, new_text: &str) -> Result<(), ServiceError> {
        node.update(new_text).map_err(|_| ServiceError::Other)?;

        self.repository
            .update_node(node)
            .map_err(|_| ServiceError::Other)?;

        Ok(())
    }
}
