use super::models;
use super::schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::io;
use std::{thread, time};

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

    pub fn wait_for_conn(
        &self,
        refresh_time: time::Duration,
        max_retries: u32,
    ) -> Result<(), io::Error> {
        for i in 0..max_retries {
            println!("Waiting for database connexion");
            match self.get_conn() {
                Ok(_conn) => {
                    println!("Database connexion is ready");
                    return Ok(());
                }
                Err(_) => println!(
                    "Database connexion is not ready yet, attempt {}/{}",
                    i, max_retries
                ),
            }
            thread::sleep(refresh_time);
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Could not connect to database after {} attempts",
                max_retries
            ),
        ))
    }

    pub async fn insert_parents(
        &self,
        conn: &PgConnection,
        new_parents: Vec<models::ParentForm>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        diesel::insert_into(schema::parents::table)
            .values(&new_parents)
            .get_results::<models::Parent>(conn)?;
        Ok(())
    }

    pub async fn insert_nodes(
        &self,
        conn: &PgConnection,
        new_nodes: Vec<models::NodeForm>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        diesel::insert_into(schema::nodes::table)
            .values(&new_nodes)
            .get_results::<models::Node>(conn)?;
        Ok(())
    }

    pub async fn update_node(
        &self,
        conn: &PgConnection,
        updated_node: models::NodeForm,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use schema::nodes::dsl::*;
        let target = nodes.filter(node.eq(&updated_node.node));
        diesel::update(target)
            .set(&updated_node)
            .get_result::<models::Node>(conn)?;
        Ok(())
    }
}
