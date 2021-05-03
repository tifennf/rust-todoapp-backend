use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub content: String,
    pub checked: String,
    pub _id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct JsonError {
    pub error: String,
}
