use super::{InterfaceError, NodeRepository};
use crate::domain::{
    Node,
    models::{NodeType, Source},
};
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
                previous_id   TEXT,
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
    fn create_node(
        &self,
        parent: Option<Uuid>,
        previous: Option<Uuid>,
        node_type: NodeType,
        text: &str,
        author: &str,
        source: Source,
    ) -> Result<Node, InterfaceError> {
        let node = Node::new(parent, previous, node_type, text, author, source)
            .map_err(|_| InterfaceError::NodeCreation)?;

        let id = node.id.to_string();
        let parent_id = node.parent_id.map(|id| id.to_string());
        let previous_id = node.previous_id.map(|id| id.to_string());
        let created_time = node.created_time.to_string();
        let modified_time = node.modified_time.to_string();
        let node_type = node.node_type.to_string();
        let source = node.source_type.to_string();

        self.connection
            .execute(
                "INSERT INTO outline (id, parent_id, previous_id, created_time, modified_time, node_type, text, author, source_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                rusqlite::params![
                    id,
                    parent_id,
                    previous_id,
                    created_time,
                    modified_time,
                    node_type,
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

    let parent_id_str: Option<String> = row
        .get(1)
        .map_err(|_| InterfaceError::FieldParseError("parent_id".to_owned()))?;

    let previous_id_str: Option<String> = row
        .get(2)
        .map_err(|_| InterfaceError::FieldParseError("previous_id".to_owned()))?;

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
        previous_id_str,
        created_time_str,
        modified_time_str,
        node_type_str,
        text,
        author,
        source_str,
    )
    .map_err(InterfaceError::Domain)
}
