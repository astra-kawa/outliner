use crate::{
    domain::{
        lexorank::to_string_padded,
        node::{CreateNodeRequest, NodeType, Source},
    },
    interfaces::{NodeRepository, SqliteRepository},
    services::{graph_service::GraphService, node_service::NodeService},
};

mod domain;
mod interfaces;
mod services;

fn main() {
    let service = NodeService {
        repository: SqliteRepository::new_memory().unwrap(),
    };

    let node1 = match service.create_node(CreateNodeRequest::new(
        None,
        &to_string_padded(100, 12),
        NodeType::Standard,
        "New node 1",
        "astra",
        Source::User,
    )) {
        Ok(node) => {
            println!("Created node: {node:?}");
            node
        }
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    match service.create_node(CreateNodeRequest::new(
        Some(node1.id()),
        &to_string_padded(100, 12),
        NodeType::Standard,
        "New node 2",
        "astra",
        Source::User,
    )) {
        Ok(node) => {
            println!("Created node: {node:?}");
            node
        }
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    match service.create_node(CreateNodeRequest::new(
        Some(node1.id()),
        &to_string_padded(500, 12),
        NodeType::Standard,
        "New node 3",
        "astra",
        Source::User,
    )) {
        Ok(node) => {
            println!("Created node: {node:?}");
            node
        }
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    // println!();
    // let mut nodes = service.repository.dump_nodes().unwrap();
    // for node in nodes.iter() {
    //     println!("Retrieved node: {node:?}")
    // }

    // println!();
    // service.delete_node(node3).unwrap();

    // nodes = service.repository.dump_nodes().unwrap();
    // for node in nodes.iter() {
    //     println!("Retrieved node: {node:?}")
    // }

    // println!();
    // let mut node1 = nodes.remove(0);
    // match service.repository.get_node(&node1.id()) {
    //     Ok(node) => println!("Got node: {node:?}"),
    //     Err(err) => eprintln!("Error: {err}"),
    // };

    // println!();
    // service
    //     .update_node(&mut node1, "Updated node 1 with new text")
    //     .unwrap();

    // match service.repository.get_node(&node1.id()) {
    //     Ok(node) => println!("Got node: {node:?}"),
    //     Err(err) => eprintln!("Error: {err}"),
    // };

    let graph = GraphService::new(service);
}
