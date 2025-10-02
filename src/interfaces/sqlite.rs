use super::{InterfaceError, NodeStore};
use crate::domain::{Node, models::Source};
use hifitime::prelude::*;
use rusqlite::{Connection, Error, Row};
use std::str::FromStr;
use uuid::Uuid;

pub struct SqliteStore {
    connection: Connection,
}

impl SqliteStore {
    pub fn new_memory() -> Result<SqliteStore, InterfaceError> {
        let connection = Connection::open_in_memory().map_err(|_| InterfaceError::DbConnection)?;

        connection
            .execute(
                "CREATE TABLE outline (
                id            TEXT PRIMARY KEY,
                parent_id     TEXT,
                previous_id   TEXT,
                created_time  TEXT,
                modified_time TEXT,
                text          TEXT,
                author        TEXT,
                source_type   TEXT
                )",
                (),
            )
            .map_err(|_| InterfaceError::TableCreation)?;

        Ok(SqliteStore { connection })
    }
}

impl NodeStore for SqliteStore {
    fn create_node(
        &self,
        parent: Option<Uuid>,
        previous: Option<Uuid>,
        text: &str,
        author: &str,
        source: Source,
    ) -> Result<Node, InterfaceError> {
        let node = Node::new(parent, previous, text, author, source)
            .map_err(|_| InterfaceError::NodeCreation)?;

        let id = node.id.to_string();
        let parent_id = node.parent_id.map(|id| id.to_string());
        let previous_id = node.previous_id.map(|id| id.to_string());
        let created_time = node.created_time.to_string();
        let modified_time = node.modified_time.to_string();
        let source = node.source_type.to_string();

        self.connection
            .execute(
                "INSERT INTO outline (id, parent_id, previous_id, created_time, modified_time, text, author, source_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                rusqlite::params![
                    id,
                    parent_id,
                    previous_id,
                    created_time,
                    modified_time,
                    &node.text,
                    &node.author,
                    source,
                ],
            )
            .map_err(|_| InterfaceError::NodeWrite)?;

        Ok(node)
    }

    fn get_node(&self, node_id: &Uuid) -> Result<Node, InterfaceError> {
        self.connection
            .prepare("SELECT * FROM outline WHERE id = ?1")
            .map_err(|_| InterfaceError::Other)?
            .query_row([node_id.to_string()], |row| {
                row_to_node(row).map_err(|_| Error::InvalidQuery)
            })
            .map_err(|err| match err {
                Error::QueryReturnedNoRows => InterfaceError::MissingNode,
                _ => InterfaceError::Other,
            })
    }

    fn update_node(&self, updated_node: &Node) -> Result<(), InterfaceError> {
        let update_row_count = self
            .connection
            .execute(
                "UPDATE outline SET text = ?1, modified_time = ?2 WHERE id = ?3",
                (
                    &updated_node.text,
                    &updated_node.modified_time.to_string(),
                    &updated_node.id.to_string(),
                ),
            )
            .map_err(|_| InterfaceError::NodeUpdate)?;

        if update_row_count == 0 {
            return Err(InterfaceError::MissingNodeOperation);
        }

        Ok(())
    }

    fn dump_nodes(&self) -> Result<Vec<Node>, InterfaceError> {
        let mut query = self
            .connection
            .prepare("SELECT * FROM outline")
            .map_err(|_| InterfaceError::Other)?;

        let nodes = query
            .query_map([], |row| row_to_node(row).map_err(|_| Error::InvalidQuery))
            .map_err(|_| InterfaceError::InvalidQuery)?;

        nodes
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| InterfaceError::Other)
    }
}

fn row_to_node(row: &Row<'_>) -> Result<Node, InterfaceError> {
    let id_str: String = row
        .get(0)
        .map_err(|_| InterfaceError::FieldParseError("id".to_owned()))?;
    let id =
        Uuid::parse_str(&id_str).map_err(|_| InterfaceError::FieldParseError("id".to_owned()))?;

    let parent_id = optional_uuid(row, 1)?;
    let previous_id = optional_uuid(row, 2)?;

    let created_str: String = row
        .get(3)
        .map_err(|_| InterfaceError::FieldParseError("created_time".to_owned()))?;
    let modified_str: String = row
        .get(4)
        .map_err(|_| InterfaceError::FieldParseError("modified_time".to_owned()))?;
    let created_time = Epoch::from_str(&created_str)
        .map_err(|_| InterfaceError::FieldParseError("created_time".to_owned()))?;
    let modified_time = Epoch::from_str(&modified_str)
        .map_err(|_| InterfaceError::FieldParseError("modified_time".to_owned()))?;

    let text: String = row
        .get(5)
        .map_err(|_| InterfaceError::FieldParseError("text".to_owned()))?;
    let author: String = row
        .get(6)
        .map_err(|_| InterfaceError::FieldParseError("author".to_owned()))?;

    let source_str: String = row
        .get(7)
        .map_err(|_| InterfaceError::FieldParseError("source".to_owned()))?;
    let source = Source::from_str(&source_str)
        .map_err(|_| InterfaceError::FieldParseError("source".to_owned()))?;

    Ok(Node {
        id,
        parent_id,
        previous_id,
        created_time,
        modified_time,
        text,
        author,
        source_type: source,
    })
}

fn optional_uuid(row: &Row<'_>, index: usize) -> Result<Option<Uuid>, InterfaceError> {
    let value: Option<String> = row.get(index).map_err(|_| InterfaceError::Other)?;
    value
        .map(|value| Uuid::parse_str(&value).map_err(|_| InterfaceError::Other))
        .transpose()
}
