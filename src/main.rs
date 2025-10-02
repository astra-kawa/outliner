use crate::interfaces::{NodeStore, SqliteStore};

mod domain;
mod interfaces;

fn main() {
    let store = SqliteStore::new_memory().unwrap();

    match store.create_node("New node 1") {
        Ok(node) => println!("Created node: {node:?}"),
        Err(err) => eprintln!("Error: {err}"),
    };

    match store.create_node("New node 2") {
        Ok(node) => println!("Created node: {node:?}"),
        Err(err) => eprintln!("Error: {err}"),
    };

    match store.create_node("New node 3") {
        Ok(node) => println!("Created node: {node:?}"),
        Err(err) => eprintln!("Error: {err}"),
    };

    println!();
    let nodes = store.dump_nodes().unwrap();
    for node in nodes.iter() {
        println!("Retrieved node: {node:?}")
    }

    println!();
    let node1_id = nodes.first().unwrap().id;
    match store.get_node(&node1_id) {
        Ok(node) => println!("Got node: {node:?}"),
        Err(err) => eprintln!("Error: {err}"),
    };
}
