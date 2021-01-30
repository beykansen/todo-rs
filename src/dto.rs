use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveTodoRequest {
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveTodoResponse {
    pub success: bool,
    pub id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateResponse {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteResponse {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllRequest {
    pub done: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllResponse<T> {
    pub todos: Vec<T>,
}