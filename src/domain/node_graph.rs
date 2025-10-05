#![allow(dead_code)]
use std::collections::HashMap;

use crate::domain::Node;
use uuid::Uuid;

#[derive(Debug)]
struct GraphElement {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub depth: usize,
    pub children: Vec<Self>,
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

            let element = construct_sub_tree(&mut node_map, None, node.id(), 0);

            graph.push(element);
        }

        Self { nodes, graph }
    }
}

fn construct_sub_tree(
    node_map: &mut HashMap<Uuid, Vec<Uuid>>,
    parent_id: Option<Uuid>,
    current_id: Uuid,
    current_depth: usize,
) -> GraphElement {
    let node_children = node_map.remove(&current_id);
    let mut child_elements: Vec<GraphElement> = Vec::new();

    if let Some(children) = node_children {
        child_elements = children
            .into_iter()
            .map(|child_id| {
                construct_sub_tree(node_map, Some(current_id), child_id, current_depth + 1)
            })
            .collect();
    };

    GraphElement {
        id: current_id,
        parent_id,
        depth: current_depth,
        children: child_elements,
    }
}

pub fn create_parent_children_map(nodes: &Vec<Node>) -> HashMap<Uuid, Vec<Uuid>> {
    let mut children_by_parent = HashMap::new();

    // basic nested for loop, improve with better logic later
    for parent_node in nodes {
        let mut children_rank_ids: Vec<(String, Uuid)> = Vec::new();

        for child_node in nodes {
            if let Some(childs_parent_id) = child_node.parent_id() {
                if parent_node.id() == childs_parent_id {
                    children_rank_ids.push((child_node.rank_key_str(), child_node.id()));
                }
            }
        }

        if !children_rank_ids.is_empty() {
            children_rank_ids.sort_by(|a, b| a.0.cmp(&b.0));
        }

        let children_ids = children_rank_ids.into_iter().map(|(_, id)| id).collect();

        children_by_parent.insert(parent_node.id(), children_ids);
    }

    children_by_parent
}
