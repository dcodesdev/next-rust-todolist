use axum::{Extension, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    app::AppState,
    error::{ApiError, ApiResult},
    middlewares::ReqUser,
    models::todo_list::TodoList,
};

#[derive(Debug, Serialize)]
pub struct Todo {
    id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct GetTodoListsResponse {
    data: Vec<TodoList>,
}

pub async fn get_todo_lists(
    Extension(user): Extension<ReqUser>,
    Extension(state): Extension<AppState>,
) -> ApiResult<Json<GetTodoListsResponse>> {
    let lists = match sqlx::query_as!(
        TodoList,
        r#"
        SELECT id, name, user_id, created_at, updated_at
        FROM lists
        WHERE user_id = $1
        "#,
        user.id
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(lists) => lists,
        Err(_) => return Err(ApiError::InternalServerError),
    };

    Ok(Json(GetTodoListsResponse { data: lists }))
}
