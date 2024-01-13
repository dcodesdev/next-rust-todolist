use axum::Json;
use serde::Serialize;

use crate::error::ApiResult;

#[derive(Debug, Serialize)]
pub struct TodoItem {
    id: u64,
    pub title: String,
    pub completed: bool,
}

pub async fn get_todos() -> ApiResult<Json<TodoItem>> {
    // [TODO]: Implement get_todos

    let todo = TodoItem {
        id: 1,
        title: "Hello".to_string(),
        completed: false,
    };

    Ok(Json(todo))
}
