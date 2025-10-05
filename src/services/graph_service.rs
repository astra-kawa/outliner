use crate::{domain::NodeGraph, interfaces::NodeRepository, services::node_service::NodeService};

pub struct GraphService<R>
where
    R: NodeRepository,
{
    graph: NodeGraph,
    node_service: NodeService<R>,
}

impl<R> GraphService<R>
where
    R: NodeRepository,
{
    pub fn new(graph: NodeGraph, node_service: NodeService<R>) -> Self {
        Self {
            graph,
            node_service,
        }
    }
}
