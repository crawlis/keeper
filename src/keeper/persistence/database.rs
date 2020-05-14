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

    fn get_conn(&self) -> PgConnection {
        PgConnection::establish(&self.uri)
            .expect(&format!("Error connecting to database {}", self.uri))
    }

    pub async fn insert(&self, new_node: models::NewNode) -> models::Node {
        println!("Insert: {:?}", new_node);
        diesel::insert_into(schema::nodes::table)
            .values(&new_node)
            .get_result(&self.get_conn())
            .expect("Error saving new node")
    }
}
