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

    pub async fn insert_parent(
        &self,
        conn: &PgConnection,
        parent_form: models::ParentForm,
    ) -> QueryResult<models::Parent> {
        use schema::parents::dsl::*;
        diesel::insert_into(parents)
            .values(parent_form)
            .on_conflict_do_nothing()
            .get_result::<models::Parent>(conn)
    }

    pub async fn insert_node(
        &self,
        conn: &PgConnection,
        node_form: models::NodeForm,
    ) -> QueryResult<models::Node> {
        use schema::nodes::dsl::*;
        diesel::insert_into(nodes)
            .values(&node_form)
            .on_conflict_do_nothing()
            .get_result::<models::Node>(conn)
    }

    pub async fn update_node(
        &self,
        conn: &PgConnection,
        node_name: &str,
        node_form: &models::NodeForm,
    ) -> QueryResult<models::Node> {
        use schema::nodes::dsl::*;
        let target = nodes.filter(node.eq(node_name));
        diesel::update(target)
            .set(node_form)
            .get_result::<models::Node>(conn)
    }
}
