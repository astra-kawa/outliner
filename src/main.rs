use crate::{
    domain::models::Source,
    interfaces::{NodeStore, SqliteStore},
};

mod domain;
mod interfaces;

fn main() {
    let store = SqliteStore::new_memory().unwrap();

    let node1 = match store.create_node(None, None, "New node 1", "astra", Source::User) {
        Ok(node) => {
            println!("Created node: {node:?}");
            node
        }
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    let node2 = match store.create_node(Some(node1.id), None, "New node 2", "astra", Source::User) {
        Ok(node) => {
            println!("Created node: {node:?}");
            node
        }
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    match store.create_node(
        Some(node1.id),
        Some(node2.id),
        "New node 3",
        "astra",
        Source::User,
    ) {
        Ok(node) => println!("Created node: {node:?}"),
        Err(err) => eprintln!("Error: {err}"),
    };

    println!();
    let mut nodes = store.dump_nodes().unwrap();
    for node in nodes.iter() {
        println!("Retrieved node: {node:?}")
    }

    println!();
    let node1 = nodes.remove(0);
    match store.get_node(&node1.id) {
        Ok(node) => println!("Got node: {node:?}"),
        Err(err) => eprintln!("Error: {err}"),
    };

    println!();
    let node1 = node1.update("Updated node 1 with new text").unwrap();
    let _ = store.update_node(&node1);

    match store.get_node(&node1.id) {
        Ok(node) => println!("Got node: {node:?}"),
        Err(err) => eprintln!("Error: {err}"),
    };
}
