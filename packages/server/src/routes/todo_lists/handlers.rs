use axum::{extract::Path, Extension, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::{
    app::AppState,
    error::{ApiError, ApiResult},
    models::{todo_item::TodoItem, todo_list::TodoList},
    routes::user::handlers::UserWithoutPassword,
};

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct GetTodoListsResponse {
    data: Vec<TodoList>,
}

pub async fn get_todo_lists(
    Extension(user): Extension<UserWithoutPassword>,
    Extension(state): Extension<AppState>,
) -> ApiResult<Json<GetTodoListsResponse>> {
    let lists = match sqlx::query_as!(
        TodoList,
        r#"
        SELECT id, name, user_id, created_at, updated_at
        FROM lists
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT 25
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

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct CreateTodoListRequest {
    name: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct CreateTodoListResponse {
    data: TodoList,
}

pub async fn create_todo_list(
    Extension(user): Extension<UserWithoutPassword>,
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

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct UpdateTodoListRequest {
    name: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct UpdateTodoListResponse {
    data: TodoList,
}

pub async fn update_todo_list(
    Path(id): Path<String>,
    Extension(user): Extension<UserWithoutPassword>,
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

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct DeleteTodoListResponse {
    #[ts(type = "string")]
    id: Uuid,
}

pub async fn delete_todo_list(
    Path(id): Path<String>,
    Extension(user): Extension<UserWithoutPassword>,
    Extension(state): Extension<AppState>,
) -> ApiResult<Json<DeleteTodoListResponse>> {
    let list_id = match Uuid::parse_str(&id) {
        Ok(list_id) => list_id,
        Err(_) => return Err(ApiError::BadRequest),
    };

    let list = match sqlx::query_as!(
        TodoList,
        r#"
        DELETE FROM lists
        WHERE user_id = $1 AND id = $2
        RETURNING id, name, user_id, created_at, updated_at
        "#,
        user.id,
        list_id
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(list) => list,
        Err(_) => return Err(ApiError::NotFound),
    };

    Ok(Json(DeleteTodoListResponse { id: list.id }))
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct GetTodoListDetailsResponse {
    list: TodoList,
    items: Vec<TodoItem>,
}

pub async fn get_todo_list_details(
    Path(id): Path<String>,
    Extension(user): Extension<UserWithoutPassword>,
    Extension(state): Extension<AppState>,
) -> ApiResult<Json<GetTodoListDetailsResponse>> {
    let list_id = match Uuid::parse_str(&id) {
        Ok(list_id) => list_id,
        Err(_) => return Err(ApiError::BadRequest),
    };

    let list = match sqlx::query_as!(
        TodoList,
        r#"
        SELECT id, name, user_id, created_at, updated_at
        FROM lists
        WHERE user_id = $1 AND id = $2
        "#,
        user.id,
        list_id
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(list) => list,
        Err(_) => return Err(ApiError::NotFound),
    };

    let items = match sqlx::query_as!(
        TodoItem,
        r#"
        SELECT id, title, description, completed, list_id, created_at, updated_at
        FROM todo_items
        WHERE list_id = $1
        ORDER BY created_at DESC
        LIMIT 25
        "#,
        list_id
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(items) => items,
        Err(_) => return Err(ApiError::InternalServerError),
    };

    Ok(Json(GetTodoListDetailsResponse { list, items }))
}
