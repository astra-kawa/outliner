use crate::{
    domain::{DomainError, NodeGraph},
    interfaces::NodeRepository,
    services::{errors::ServiceError, logging::LoggingService, node_service::NodeService},
};

pub struct GraphService<R, L>
where
    R: NodeRepository,
    L: LoggingService,
{
    pub node_graph: NodeGraph,
    pub node_service: NodeService<R, L>,
}

impl<R, L> GraphService<R, L>
where
    R: NodeRepository,
    L: LoggingService,
{
    pub fn new(node_service: NodeService<R, L>) -> Result<GraphService<R, L>, ServiceError> {
        let nodes = node_service.dump_nodes()?;
        let graph = NodeGraph::new(nodes);

        Ok(Self {
            node_graph: graph,
            node_service,
        })
    }
}
