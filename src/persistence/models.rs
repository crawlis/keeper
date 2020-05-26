use super::schema::nodes;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "nodes"]
pub struct NewNode {
    pub parent: String,
    pub value: String,
}

#[derive(Queryable, Serialize)]
pub struct Node {
    pub id: i32,
    pub parent: String,
    pub value: String,
}
