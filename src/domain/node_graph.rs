use crate::domain::Node;
use std::collections::HashMap;
use uuid::Uuid;

pub struct NodeGraph {
    pub nodes: Vec<Node>,
    pub children_by_parent: HashMap<Uuid, Vec<Uuid>>,
}

impl NodeGraph {
    pub fn new(nodes: Vec<Node>) -> Self {
        let mut children_by_parent = HashMap::new();

        // basic nested for loop, improve with better logic later
        for parent_node in &nodes {
            let mut children_rank_ids: Vec<(String, Uuid)> = Vec::new();

            for child_node in &nodes {
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

        Self {
            nodes,
            children_by_parent,
        }
    }
}
