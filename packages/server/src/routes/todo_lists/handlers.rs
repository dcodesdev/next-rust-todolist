use axum::{extract::Path, Extension, Json};
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

#[derive(Debug, Deserialize)]
pub struct UpdateTodoListRequest {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateTodoListResponse {
    data: TodoList,
}

pub async fn update_todo_list(
    Path(id): Path<String>,
    Extension(user): Extension<ReqUser>,
    Extension(state): Extension<AppState>,
    Json(body): Json<UpdateTodoListRequest>,
) -> ApiResult<Json<UpdateTodoListResponse>> {
    let list_id = match Uuid::parse_str(&id) {
        Ok(list_id) => list_id,
        Err(_) => return Err(ApiError::BadRequest),
    };

    let list = match sqlx::query_as!(
        TodoList,
        r#"
        UPDATE lists
        SET name = $1
        WHERE user_id = $2 AND id = $3
        RETURNING id, name, user_id, created_at, updated_at
        "#,
        body.name,
        user.id,
        list_id
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(list) => list,
        Err(_) => return Err(ApiError::NotFound),
    };

    Ok(Json(UpdateTodoListResponse { data: list }))
}
