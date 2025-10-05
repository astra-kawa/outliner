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
    pub fn new(node_service: NodeService<R>) -> Self {
        let nodes = node_service.dump_nodes().unwrap();
        let graph = NodeGraph::new(nodes);

        Self {
            graph,
            node_service,
        }
    }
}
