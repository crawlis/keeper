use super::schema::{nodes, parents};
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Deserialize, AsChangeset, Debug)]
#[table_name = "nodes"]
pub struct NodeForm {
    pub node: String,
    pub visited: bool,
}

#[derive(Queryable, Serialize)]
pub struct Node {
    pub id: i32,
    pub node: String,
    pub visited: bool,
}

#[derive(Insertable, Deserialize, AsChangeset, Debug)]
#[table_name = "parents"]
pub struct ParentForm {
    pub parent: String,
    pub node: String,
}

#[derive(Queryable, Serialize)]
pub struct Parent {
    pub id: i32,
    pub parent: String,
    pub node: String,
}
