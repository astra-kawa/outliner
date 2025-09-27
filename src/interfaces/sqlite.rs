use rusqlite::Connection;

use super::{InterfaceError, NodeStore};
use crate::domain::Node;

pub struct SqliteStore {
    connection: Connection,
}

impl SqliteStore {
    pub fn new_memory() -> Result<SqliteStore, InterfaceError> {
        let connection = match Connection::open_in_memory() {
            Ok(connection) => connection,
            Err(_) => return Err(InterfaceError::Other),
        };

        let table_result = connection.execute(
            "CREATE TABLE person (
                id   TEXT PRIMARY KEY,
                text TEXT
            )",
            (),
        );

        if table_result.is_err() {
            return Err(InterfaceError::Other);
        }

        Ok(SqliteStore { connection })
    }
}

impl NodeStore for SqliteStore {
    fn create_node(&self, text: &str) -> Result<Node, InterfaceError> {
        let node = match Node::new(text) {
            Ok(node) => node,
            Err(_) => return Err(InterfaceError::Other),
        };

        let db_write_result = self.connection.execute(
            "INSERT INTO person (id, text) VALUES (?1, ?2)",
            (node.id.to_string(), &node.text),
        );

        if db_write_result.is_err() {
            return Err(InterfaceError::Other);
        }

        Ok(node)
    }
}
