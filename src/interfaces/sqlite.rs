use super::{InterfaceError, NodeRepository};
use crate::domain::Node;
use rusqlite::{Connection, Error, Row};
use uuid::Uuid;

pub struct SqliteRepository {
    connection: Connection,
}

impl SqliteRepository {
    pub fn new_memory() -> Result<SqliteRepository, InterfaceError> {
        let connection = Connection::open_in_memory().map_err(|_| InterfaceError::DbConnection)?;

        connection
            .execute(
                "CREATE TABLE outline (
                id            TEXT PRIMARY KEY,
                parent_id     TEXT,
                rank_key      TEXT,
                created_time  TEXT,
                modified_time TEXT,
                node_type     TEXT,
                text          TEXT,
                author        TEXT,
                source_type   TEXT
                )",
                (),
            )
            .map_err(|_| InterfaceError::TableCreation)?;

        Ok(SqliteRepository { connection })
    }
}

impl NodeRepository for SqliteRepository {
    fn add_node(&self, node: &Node) -> Result<(), InterfaceError> {
        self.connection
            .execute(
                "INSERT INTO outline (id, parent_id, rank_key, created_time, modified_time, node_type, text, author, source_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                rusqlite::params![
                    node.id_str(),
                    node.parent_id_str(),
                    node.rank_key_str(),
                    node.created_time_str(),
                    node.modified_time_str(),
                    node.node_type_str(),
                    node.text(),
                    node.author(),
                    node.source_type_str(),
                ],
            )
            .map_err(|_| InterfaceError::NodeWrite)?;

        Ok(())
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
                    updated_node.text(),
                    updated_node.modified_time_str(),
                    updated_node.id_str(),
                ),
            )
            .map_err(|_| InterfaceError::NodeUpdate)?;

        if update_row_count == 0 {
            return Err(InterfaceError::MissingNodeOperation);
        }

        Ok(())
    }

    fn delete_node(&self, node_id: &Uuid) -> Result<(), InterfaceError> {
        let delete_row_count = self
            .connection
            .execute("DELETE FROM outline WHERE id = ?1", (node_id.to_string(),))
            .map_err(|_| InterfaceError::NodeDelete)?;

        if delete_row_count == 0 {
            return Err(InterfaceError::MissingNode);
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

    let parent_id_str: Option<String> = row
        .get(1)
        .map_err(|_| InterfaceError::FieldParseError("parent_id".to_owned()))?;

    let rank_key: String = row
        .get(2)
        .map_err(|_| InterfaceError::FieldParseError("rank_key".to_owned()))?;

    let created_time_str: String = row
        .get(3)
        .map_err(|_| InterfaceError::FieldParseError("created_time".to_owned()))?;

    let modified_time_str: String = row
        .get(4)
        .map_err(|_| InterfaceError::FieldParseError("modified_time".to_owned()))?;

    let node_type_str: String = row
        .get(5)
        .map_err(|_| InterfaceError::FieldParseError("node_type".to_owned()))?;

    let text: String = row
        .get(6)
        .map_err(|_| InterfaceError::FieldParseError("text".to_owned()))?;

    let author: String = row
        .get(7)
        .map_err(|_| InterfaceError::FieldParseError("author".to_owned()))?;

    let source_str: String = row
        .get(8)
        .map_err(|_| InterfaceError::FieldParseError("source".to_owned()))?;

    Node::from_raw_strs(
        id_str,
        parent_id_str,
        rank_key,
        created_time_str,
        modified_time_str,
        node_type_str,
        text,
        author,
        source_str,
    )
    .map_err(InterfaceError::Domain)
}
