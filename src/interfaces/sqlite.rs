use super::{InterfaceError, NodeStore};
use crate::domain::{Node, models::Source};
use core::str::FromStr;
use hifitime::prelude::*;
use rusqlite::{Connection, Error, Row};
use uuid::Uuid;

pub struct SqliteStore {
    connection: Connection,
}

impl SqliteStore {
    pub fn new_memory() -> Result<SqliteStore, InterfaceError> {
        let connection = Connection::open_in_memory().map_err(|_| InterfaceError::Other)?;

        connection
            .execute(
                "CREATE TABLE outline (
                id            TEXT PRIMARY KEY,
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
    fn create_node(&self, text: &str) -> Result<Node, InterfaceError> {
        let node =
            Node::new(text, "author", Source::User).map_err(|_| InterfaceError::NodeCreation)?;

        self.connection
            .execute(
                "INSERT INTO outline (id, created_time, modified_time, text, author, source_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                (
                    &node.id.to_string(),
                    &node.created_time.to_string(),
                    &node.modified_time.to_string(),
                    &node.text,
                    &node.author,
                    format!("{:?}", node.source_type)
                ),
            )
            .map_err(|_| InterfaceError::NodeWrite)?;

        Ok(node)
    }

    fn get_node(&self, node_id: &Uuid) -> Result<Node, InterfaceError> {
        let mut query = self
            .connection
            .prepare("SELECT * FROM outline WHERE id = ?1")
            .map_err(|_| InterfaceError::Other)?;

        query
            .query_one([node_id.to_string()], |row| match row_to_node(row) {
                Ok(node) => Ok(node),
                Err(_) => Err(Error::InvalidQuery),
            })
            .map_err(|_| InterfaceError::Other)
    }

    fn dump_nodes(&self) -> Result<Vec<Node>, InterfaceError> {
        let mut query = self
            .connection
            .prepare("SELECT * FROM outline")
            .map_err(|_| InterfaceError::Other)?;

        let query_result = query
            .query_map([], |row| match row_to_node(row) {
                Ok(node) => Ok(node),
                Err(_) => Err(Error::InvalidQuery),
            })
            .map_err(|_| InterfaceError::Other)?;

        let mut nodes: Vec<Node> = Vec::new();
        for node_result in query_result {
            nodes.push(node_result.unwrap());
        }

        Ok(nodes)
    }
}

fn row_to_node(row: &Row<'_>) -> Result<Node, InterfaceError> {
    let id_str: String = row.get(0).map_err(|_| InterfaceError::Other)?;
    let created_str: String = row.get(1).map_err(|_| InterfaceError::Other)?;
    let modified_str: String = row.get(2).map_err(|_| InterfaceError::Other)?;

    let source_str: String = row.get(5).map_err(|_| InterfaceError::Other)?;
    let source = Source::from_str(&source_str).map_err(|_| InterfaceError::Other)?;

    Ok(Node {
        id: Uuid::parse_str(&id_str).unwrap(),
        created_time: Epoch::from_str(&created_str).unwrap(),
        modified_time: Epoch::from_str(&modified_str).unwrap(),
        text: row.get(3).unwrap(),
        author: row.get(4).unwrap(),
        source_type: source,
    })
}
