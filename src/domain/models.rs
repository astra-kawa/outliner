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
    pub node_type: NodeType,
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

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Standard,
    Todo,
    InProgress,
    Done,
}

impl FromStr for NodeType {
    type Err = ();

    fn from_str(input: &str) -> Result<NodeType, Self::Err> {
        match input.to_ascii_lowercase().as_str() {
            "standard" => Ok(NodeType::Standard),
            "todo" => Ok(NodeType::Todo),
            "inprogress" => Ok(NodeType::InProgress),
            "done" => Ok(NodeType::Done),
            _ => Err(()),
        }
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            NodeType::Standard => "Standard",
            NodeType::Todo => "Todo",
            NodeType::InProgress => "InProgress",
            NodeType::Done => "Done",
        };

        f.write_str(label)
    }
}

impl Node {
    pub fn new(request: CreateNodeRequest) -> Result<Self, DomainError> {
        let now = Epoch::now().map_err(|_| DomainError::InvalidDateTime)?;

        Ok(Node {
            id: Uuid::new_v4(),
            parent_id: request.parent_id,
            previous_id: request.previous_id,
            created_time: now,
            modified_time: now,
            node_type: request.node_type,
            text: request.text,
            author: request.author,
            source_type: request.source_type,
        })
    }

    pub fn from_raw_strs(
        id_str: String,
        parent_id_str: Option<String>,
        previous_id_str: Option<String>,
        created_time_str: String,
        modified_time_str: String,
        node_type_str: String,
        text: String,
        author: String,
        source_type_str: String,
    ) -> Result<Self, DomainError> {
        let id = Uuid::parse_str(&id_str).map_err(|_| DomainError::FieldParseError("id".into()))?;

        let parent_id = match parent_id_str {
            Some(str) => match Uuid::parse_str(&str) {
                Ok(id) => Some(id),
                Err(_) => return Err(DomainError::FieldParseError("parent_id".into())),
            },
            None => None,
        };

        let previous_id = match previous_id_str {
            Some(str) => match Uuid::parse_str(&str) {
                Ok(id) => Some(id),
                Err(_) => return Err(DomainError::FieldParseError("previous_id".into())),
            },
            None => None,
        };

        let created_time = Epoch::from_str(&created_time_str)
            .map_err(|_| DomainError::FieldParseError("created_time".into()))?;

        let modified_time = Epoch::from_str(&modified_time_str)
            .map_err(|_| DomainError::FieldParseError("modified_time".into()))?;

        let node_type = NodeType::from_str(&node_type_str)
            .map_err(|_| DomainError::FieldParseError("node_type".into()))?;

        let source_type = Source::from_str(&source_type_str)
            .map_err(|_| DomainError::FieldParseError("source".to_owned()))?;

        Ok(Node {
            id,
            parent_id,
            previous_id,
            created_time,
            modified_time,
            node_type,
            text,
            author,
            source_type,
        })
    }

    pub fn update(&mut self, text: impl Into<String>) -> Result<(), DomainError> {
        self.text = text.into();
        self.modified_time = Epoch::now().map_err(|_| DomainError::InvalidDateTime)?;

        Ok(())
    }
}

pub struct CreateNodeRequest {
    parent_id: Option<Uuid>,
    previous_id: Option<Uuid>,
    node_type: NodeType,
    text: String,
    author: String,
    source_type: Source,
}

impl CreateNodeRequest {
    pub fn new(
        parent_id: Option<Uuid>,
        previous_id: Option<Uuid>,
        node_type: NodeType,
        text: &str,
        author: &str,
        source_type: Source,
    ) -> Self {
        CreateNodeRequest {
            parent_id,
            previous_id,
            node_type,
            text: text.into(),
            author: author.into(),
            source_type,
        }
    }
}
