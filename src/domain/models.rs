use std::str::FromStr;

use crate::domain::DomainError;
use hifitime::Epoch;
use uuid::Uuid;

#[derive(Debug)]
pub struct Node {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub previous_id: Option<Uuid>,
    pub created_time: Epoch,
    pub modified_time: Epoch,
    pub text: String,
    pub author: String,
    pub source_type: Source,
}

#[derive(Debug, PartialEq)]
pub enum Source {
    User,
    Agent,
    Application,
}

impl FromStr for Source {
    type Err = ();

    fn from_str(input: &str) -> Result<Source, Self::Err> {
        match input {
            "User" | "USER" | "user" => Ok(Source::User),
            "Agent" | "AGENT" | "agent" => Ok(Source::Agent),
            "Application" | "APPLICATION" | "application" => Ok(Source::Application),
            _ => Err(()),
        }
    }
}

impl Node {
    pub fn new(
        parent: Option<Uuid>,
        previous: Option<Uuid>,
        text: &str,
        author: &str,
        source_type: Source,
    ) -> Result<Self, DomainError> {
        let now = Epoch::now().map_err(|_| DomainError::Other)?;

        Ok(Node {
            id: Uuid::new_v4(),
            parent_id: parent,
            previous_id: previous,
            created_time: now,
            modified_time: now,
            text: text.into(),
            author: author.into(),
            source_type,
        })
    }

    pub fn update(self, text: &str) -> Result<Self, DomainError> {
        // self.text = text.to_owned();
        // self.modified_time = Epoch::now().map_err(|_| DomainError::Other)?;

        // Ok(self)
        let new_time = Epoch::now().map_err(|_| DomainError::Other)?;

        Ok(Node {
            text: text.to_owned(),
            modified_time: new_time,
            ..self
        })
    }
}
