use actix_web::web;
use random_number::random_ranged;
use std::{collections::HashMap, fs, sync::Mutex};

use super::schemas::Todo;

pub struct JsonDb {
    pub content: Mutex<HashMap<String, Todo>>,
}

pub fn random_id() -> i32 {
    random_ranged(1..=10000)
}

// pub fn todo_hashmap_vec(hashmap: &HashMap<String, Todo>) -> Vec<Todo> {

// 	let mut vec = Vec::new();

// 	for (_id, todo) in hashmap.iter() {
// 		vec.push(todo.clone());
// 	}

// 	vec
// }

pub async fn update_db(db: String) {
    web::block(|| fs::write("./db.json", db)).await.unwrap();
}
