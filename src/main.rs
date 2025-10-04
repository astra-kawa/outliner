use crate::{
    domain::models::{NodeType, Source},
    interfaces::{NodeRepository, SqliteRepository},
    services::node_service::{NodeService, Service},
};

mod domain;
mod interfaces;
mod services;

fn main() {
    let service = Service {
        repository: SqliteRepository::new_memory().unwrap(),
    };

    let node1 = match service.repository.create_node(
        None,
        None,
        NodeType::Standard,
        "New node 1",
        "astra",
        Source::User,
    ) {
        Ok(node) => {
            println!("Created node: {node:?}");
            node
        }
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    let node2 = match service.repository.create_node(
        Some(node1.id),
        None,
        NodeType::Standard,
        "New node 2",
        "astra",
        Source::User,
    ) {
        Ok(node) => {
            println!("Created node: {node:?}");
            node
        }
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    match service.repository.create_node(
        Some(node1.id),
        Some(node2.id),
        NodeType::Standard,
        "New node 3",
        "astra",
        Source::User,
    ) {
        Ok(node) => println!("Created node: {node:?}"),
        Err(err) => eprintln!("Error: {err}"),
    };

    println!();
    let mut nodes = service.repository.dump_nodes().unwrap();
    for node in nodes.iter() {
        println!("Retrieved node: {node:?}")
    }

    println!();
    let mut node1 = nodes.remove(0);
    match service.repository.get_node(&node1.id) {
        Ok(node) => println!("Got node: {node:?}"),
        Err(err) => eprintln!("Error: {err}"),
    };

    println!();
    service
        .update_node(&mut node1, "Updated node 1 with new text")
        .unwrap();

    match service.repository.get_node(&node1.id) {
        Ok(node) => println!("Got node: {node:?}"),
        Err(err) => eprintln!("Error: {err}"),
    };
}
