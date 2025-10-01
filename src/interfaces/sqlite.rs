use super::{InterfaceError, NodeStore};
use crate::domain::Node;
use rusqlite::Connection;
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
                id   TEXT PRIMARY KEY,
                text TEXT
                )",
                (),
            )
            .map_err(|_| InterfaceError::Other)?;

        Ok(SqliteStore { connection })
    }
}

impl NodeStore for SqliteStore {
    fn create_node(&self, text: &str) -> Result<Node, InterfaceError> {
        let node = Node::new(text).map_err(|_| InterfaceError::Other)?;

        self.connection
            .execute(
                "INSERT INTO person (id, text) VALUES (?1, ?2)",
                (node.id.to_string(), &node.text),
            )
            .map_err(|_| InterfaceError::Other)?;

        Ok(node)
    }

    fn dump_nodes(&self) -> Result<Vec<Node>, InterfaceError> {
        let mut query = self
            .connection
            .prepare("SELECT id, text FROM outline")
            .map_err(|_| InterfaceError::Other)?;

        let query_result = query
            .query_map([], |row| {
                let id_txt: String = row.get(0)?;

                Ok(Node {
                    id: Uuid::parse_str(&id_txt).unwrap(),
                    text: row.get(1).unwrap(),
                })
            })
            .map_err(|_| InterfaceError::Other)?;

        let mut nodes: Vec<Node> = Vec::new();
        for node_result in query_result {
            nodes.push(node_result.unwrap());
        }

        Ok(nodes)
    }
}
