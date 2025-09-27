use uuid::Uuid;

use crate::domain::DomainError;

pub struct Node {
    id: Uuid,
    text: String,
}

impl Node {
    pub fn new(text: &str) -> Result<Node, DomainError> {
        Ok(Node {
            id: Uuid::new_v4(),
            text: text.into(),
        })
    }
}
