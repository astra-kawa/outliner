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
                parent_id     TEXT,
                next_id       TEXT,
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
        let node = Node::new(None, None, text, "author", Source::User)
            .map_err(|_| InterfaceError::NodeCreation)?;

        let parent_str = match node.parent_id {
            Some(id) => id.to_string(),
            None => "".to_string(),
        };

        let next_str = match node.next_id {
            Some(id) => id.to_string(),
            None => "".to_string(),
        };

        self.connection
            .execute(
                "INSERT INTO outline (id, parent_id, next_id, created_time, modified_time, text, author, source_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                (
                    &node.id.to_string(),
                    &parent_str,
                    &next_str,
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

    let parent_str: String = row.get(1).map_err(|_| InterfaceError::Other)?;
    let parent_id = match parent_str.as_str() {
        "" => None,
        _ => Some(Uuid::parse_str(&parent_str).unwrap()),
    };

    let next_str: String = row.get(2).map_err(|_| InterfaceError::Other)?;
    let next_id = match next_str.as_str() {
        "" => None,
        _ => Some(Uuid::parse_str(&next_str).unwrap()),
    };

    let created_str: String = row.get(3).map_err(|_| InterfaceError::Other)?;
    let modified_str: String = row.get(4).map_err(|_| InterfaceError::Other)?;

    let source_str: String = row.get(7).map_err(|_| InterfaceError::Other)?;
    let source = Source::from_str(&source_str).map_err(|_| InterfaceError::Other)?;

    Ok(Node {
        id: Uuid::parse_str(&id_str).unwrap(),
        parent_id,
        next_id,
        created_time: Epoch::from_str(&created_str).unwrap(),
        modified_time: Epoch::from_str(&modified_str).unwrap(),
        text: row.get(5).unwrap(),
        author: row.get(6).unwrap(),
        source_type: source,
    })
}
