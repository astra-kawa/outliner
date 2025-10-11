use crate::{
    domain::node::{CreateNodeRequest, NodeType, Source},
    interfaces::SqliteRepository,
    services::{graph_service::GraphService, logging::TerminalLogging, node_service::NodeService},
};

mod domain;
mod interfaces;
mod services;

fn main() {
    let service = NodeService {
        repository: SqliteRepository::new_memory().unwrap(),
        logger: TerminalLogging::new(),
    };

    let node1 = match service.create_node(CreateNodeRequest::new(
        None,
        100,
        NodeType::Standard,
        "New node 1",
        "astra",
        Source::User,
    )) {
        Ok(node) => node,
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    match service.create_node(CreateNodeRequest::new(
        Some(node1.id()),
        100,
        NodeType::Standard,
        "New node 2",
        "astra",
        Source::User,
    )) {
        Ok(node) => node,
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    match service.create_node(CreateNodeRequest::new(
        Some(node1.id()),
        200,
        NodeType::Standard,
        "New node 3",
        "astra",
        Source::User,
    )) {
        Ok(node) => node,
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    service
        .create_node(CreateNodeRequest::new(
            Some(node1.id()),
            300,
            NodeType::Standard,
            "New node 4",
            "astra",
            Source::User,
        ))
        .unwrap();

    service
        .create_node(CreateNodeRequest::new(
            Some(node1.id()),
            400,
            NodeType::Standard,
            "New node 5",
            "astra",
            Source::User,
        ))
        .unwrap();

    service
        .create_node(CreateNodeRequest::new(
            Some(node1.id()),
            500,
            NodeType::Standard,
            "New node 6",
            "astra",
            Source::User,
        ))
        .unwrap();

    let graph_service = GraphService::new(service).unwrap();

    for element in graph_service.node_graph.graph {
        for node in element.children {
            println!("{node:?}");
        }
    }
}
