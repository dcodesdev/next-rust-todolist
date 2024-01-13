use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Deserialize)]
pub struct CreateTodoListRequest {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateTodoListResponse {
    data: TodoList,
}

pub async fn create_todo_list(
    Extension(user): Extension<ReqUser>,
    Extension(state): Extension<AppState>,
    Json(body): Json<CreateTodoListRequest>,
) -> ApiResult<Json<CreateTodoListResponse>> {
    let list = match sqlx::query_as!(
        TodoList,
        r#"
        INSERT INTO lists (name, user_id)
        VALUES ($1, $2)
        RETURNING id, name, user_id, created_at, updated_at
        "#,
        body.name,
        user.id
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(list) => list,
        Err(_) => return Err(ApiError::InternalServerError),
    };

    Ok(Json(CreateTodoListResponse { data: list }))
}
