use hifitime::Epoch;

use crate::{
    domain::{Node, node::CreateNodeRequest},
    interfaces::NodeRepository,
    services::{errors::ServiceError, logging::LoggingService},
};

pub struct NodeService<R, L>
where
    R: NodeRepository,
    L: LoggingService,
{
    pub repository: R,
    pub logger: L,
}

impl<R, L> NodeService<R, L>
where
    R: NodeRepository,
    L: LoggingService,
{
    pub fn create_node(&self, request: CreateNodeRequest) -> Result<Node, ServiceError> {
        let node = Node::new(request).map_err(ServiceError::Domain)?;

        self.repository
            .add_node(&node)
            .map_err(ServiceError::Interface)?;

        self.logger.write_log(format!(
            "{} | Created node: {}",
            node.created_time_str(),
            node.id_str()
        ))?;

        Ok(node)
    }

    pub fn update_node(&self, node: &mut Node, new_text: &str) -> Result<(), ServiceError> {
        node.update(new_text).map_err(ServiceError::Domain)?;

        self.repository
            .update_node(node)
            .map_err(ServiceError::Interface)?;

        self.logger.write_log(format!(
            "{} | Updated node: {}",
            node.modified_time_str(),
            node.id_str()
        ))?;

        Ok(())
    }

    pub fn delete_node(&self, node: Node) -> Result<(), ServiceError> {
        // todo: figure out how to handle deleting a node when it contains children
        // additionally, if deleted node has a sibling, update the next node's previous_id
        self.repository
            .delete_node(&node.id())
            .map_err(ServiceError::Interface)?;

        self.logger.write_log(format!(
            "{} | Deleted node: {}",
            Epoch::now().unwrap(),
            node.id_str()
        ))?;

        Ok(())
    }

    pub fn dump_nodes(&self) -> Result<Vec<Node>, ServiceError> {
        self.repository
            .dump_nodes()
            .map_err(ServiceError::Interface)
    }
}
