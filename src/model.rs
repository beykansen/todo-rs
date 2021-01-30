use bson::DateTime;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: String,
    pub name: String,
    pub done: bool,
    pub added_at: DateTime,
    pub tags: Vec<String>,
}