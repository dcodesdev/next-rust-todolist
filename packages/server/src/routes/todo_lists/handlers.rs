use axum::Json;
use serde::Serialize;
use uuid::Uuid;

use crate::error::ApiResult;

#[derive(Debug, Serialize)]
pub struct Todo {
    id: Uuid,
}

pub async fn get_todo_lists() -> ApiResult<Json<Todo>> {
    let todo = Todo { id: Uuid::new_v4() };

    Ok(Json(todo))
}
