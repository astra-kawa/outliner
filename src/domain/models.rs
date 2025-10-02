use crate::domain::DomainError;
use hifitime::Epoch;
use std::{fmt, str::FromStr};
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
        match input.to_ascii_lowercase().as_str() {
            "user" => Ok(Source::User),
            "agent" => Ok(Source::Agent),
            "application" => Ok(Source::Application),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Source::User => "User",
            Source::Agent => "Agent",
            Source::Application => "Application",
        };

        f.write_str(label)
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
        let now = Epoch::now().map_err(|_| DomainError::InvalidDateTime)?;

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

    pub fn update(mut self, text: impl Into<String>) -> Result<Self, DomainError> {
        self.text = text.into();
        self.modified_time = Epoch::now().map_err(|_| DomainError::InvalidDateTime)?;

        Ok(self)
    }
}
