use super::models;
use super::schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
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
    ) -> Result<(), String> {
        for i in 0..max_retries {
            println!("Waiting for database connexion, attempt number: {}", i);
            match self.get_conn() {
                Ok(_conn) => {
                    println!("Database connexion is ready");
                    return Ok(());
                }
                Err(_) => println!("Database connexion is not ready yet"),
            }
            thread::sleep(refresh_time);
        }
        Err(format!(
            "Could not connect to database after {} attempts",
            max_retries
        ))
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
