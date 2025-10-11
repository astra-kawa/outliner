#![allow(dead_code)]
use std::collections::HashMap;

use crate::domain::Node;
use uuid::Uuid;

#[derive(Debug)]
pub struct GraphElement {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub rank: u64,
    pub depth: usize,
    pub children: Vec<GraphElement>,
}

pub struct NodeGraph {
    pub nodes: Vec<Node>,
    pub graph: Vec<GraphElement>,
}

impl NodeGraph {
    pub fn new(nodes: Vec<Node>) -> Self {
        let mut graph: Vec<GraphElement> = Vec::new();
        let mut node_map = create_parent_children_map(&nodes);

        for node in &nodes {
            // only want to consider root nodes
            if node.parent_id().is_some() {
                continue;
            }

            let element = construct_sub_tree(&mut node_map, None, node.id(), node.rank(), 0);

            graph.push(element);
        }

        Self { nodes, graph }
    }
}

fn construct_sub_tree(
    node_map: &mut HashMap<Uuid, Vec<(Uuid, u64)>>,
    parent_id: Option<Uuid>,
    current_id: Uuid,
    current_rank: u64,
    current_depth: usize,
) -> GraphElement {
    let node_children = node_map.remove(&current_id);
    let mut child_elements: Vec<GraphElement> = Vec::new();

    if let Some(children) = node_children {
        child_elements = children
            .into_iter()
            .map(|(child_id, child_rank)| {
                construct_sub_tree(
                    node_map,
                    Some(current_id),
                    child_id,
                    child_rank,
                    current_depth + 1,
                )
            })
            .collect();
    };

    GraphElement {
        id: current_id,
        parent_id,
        rank: current_rank,
        depth: current_depth,
        children: child_elements,
    }
}

pub fn create_parent_children_map(nodes: &Vec<Node>) -> HashMap<Uuid, Vec<(Uuid, u64)>> {
    let mut children_by_parent = HashMap::new();

    // basic nested for loop, improve with better logic later
    for parent_node in nodes {
        let mut children_rank_ids: Vec<(Uuid, u64)> = Vec::new();

        for child_node in nodes {
            if let Some(childs_parent_id) = child_node.parent_id() {
                if parent_node.id() == childs_parent_id {
                    children_rank_ids.push((child_node.id(), child_node.rank()));
                }
            }
        }

        if !children_rank_ids.is_empty() {
            children_rank_ids.sort_by(|a, b| a.1.cmp(&b.1));
        }

        children_by_parent.insert(parent_node.id(), children_rank_ids);
    }

    children_by_parent
}
