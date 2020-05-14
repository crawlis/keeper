use super::models;
use super::schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Clone)]
pub struct Database {
    uri: String,
}

impl Database {
    pub fn new(uri: &str) -> Database {
        Database {
            uri: String::from(uri),
        }
    }

    pub fn run_migrations(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.get_conn()?;
        embed_migrations!();
        embedded_migrations::run(&conn)?;
        Ok(())
    }

    pub fn get_conn(&self) -> Result<PgConnection, diesel::ConnectionError> {
        let conn = PgConnection::establish(&self.uri)?;
        Ok(conn)
    }

    pub async fn insert(
        &self,
        new_node: models::NewNode,
    ) -> Result<models::Node, Box<dyn std::error::Error>> {
        let conn = self.get_conn()?;
        let node = diesel::insert_into(schema::nodes::table)
            .values(&new_node)
            .get_result(&conn)?;
        Ok(node)
    }
}
