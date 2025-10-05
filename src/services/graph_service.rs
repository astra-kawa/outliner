use crate::{
    domain::NodeGraph,
    interfaces::NodeRepository,
    services::{logging::LoggingService, node_service::NodeService},
};

pub struct GraphService<R, L>
where
    R: NodeRepository,
    L: LoggingService,
{
    graph: NodeGraph,
    node_service: NodeService<R, L>,
}

impl<R, L> GraphService<R, L>
where
    R: NodeRepository,
    L: LoggingService,
{
    pub fn new(node_service: NodeService<R, L>) -> Self {
        let nodes = node_service.dump_nodes().unwrap();
        let graph = NodeGraph::new(nodes);

        Self {
            graph,
            node_service,
        }
    }
}
