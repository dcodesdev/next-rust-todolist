use axum::{Extension, Json};
use serde::Serialize;

use crate::{
    app::AppState,
    error::{ApiError, ApiResult},
    middlewares::ReqUser,
    models::todo_item::TodoItem,
};

#[derive(Debug, Serialize)]
pub struct GetTodosResponse {
    pub data: Vec<TodoItem>,
}

pub async fn get_todos(
    Extension(state): Extension<AppState>,
    Extension(user): Extension<ReqUser>,
) -> ApiResult<Json<GetTodosResponse>> {
    let todos = match sqlx::query_as!(
        TodoItem,
        r#"
        SELECT todo_items.id, title, description, completed, list_id,
        todo_items.created_at, todo_items.updated_at
        FROM todo_items
        INNER JOIN lists ON lists.id = todo_items.list_id
        WHERE lists.user_id = $1
        "#,
        &user.id
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(todos) => todos,
        Err(_) => return Err(ApiError::InternalServerError),
    };

    Ok(Json(GetTodosResponse { data: todos }))
}
