use axum::{extract::Path, Extension, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::AppState,
    error::{ApiError, ApiResult},
    models::todo_item::TodoItem,
    routes::user::handlers::UserWithoutPassword,
};

#[derive(Debug, Serialize)]
pub struct GetTodosResponse {
    pub data: Vec<TodoItem>,
}

pub async fn get_todos(
    Extension(state): Extension<AppState>,
    Extension(user): Extension<UserWithoutPassword>,
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

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
    pub list_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct CreateTodoResponse {
    pub data: TodoItem,
}

pub async fn create_todo(
    Extension(state): Extension<AppState>,
    Extension(user): Extension<UserWithoutPassword>,
    Json(body): Json<CreateTodoRequest>,
) -> ApiResult<Json<CreateTodoResponse>> {
    let todo = match sqlx::query_as!(
        TodoItem,
        r#"
        INSERT INTO todo_items (title, description, list_id)
        SELECT $1, $2, $3
        FROM lists
        WHERE lists.id = $3 AND lists.user_id = $4
        RETURNING todo_items.id, todo_items.title, todo_items.description, 
        todo_items.completed, todo_items.list_id, todo_items.created_at, todo_items.updated_at
        "#,
        &body.title,
        body.description,
        &body.list_id,
        user.id // Assuming user.id is the field with the user's ID
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(todo) => todo,
        Err(_) => return Err(ApiError::InternalServerError),
    };

    Ok(Json(CreateTodoResponse { data: todo }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UpdateTodoResponse {
    pub data: TodoItem,
}

pub async fn update_todo(
    Extension(state): Extension<AppState>,
    Extension(user): Extension<UserWithoutPassword>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTodoRequest>,
) -> ApiResult<Json<UpdateTodoResponse>> {
    let todo = match sqlx::query_as!(
        TodoItem,
        r#"
        UPDATE todo_items
        SET title = COALESCE($1, title), description = COALESCE($2, description),
        completed = COALESCE($3, completed)
        FROM lists
        WHERE todo_items.id = $4 AND todo_items.list_id = lists.id AND lists.user_id = $5
        RETURNING todo_items.id, todo_items.title, todo_items.description, 
        todo_items.completed, todo_items.list_id, todo_items.created_at, todo_items.updated_at
        "#,
        body.title,
        body.description,
        body.completed,
        id,
        user.id // Assuming user.id is the field with the user's ID
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(todo) => todo,
        Err(_) => return Err(ApiError::InternalServerError),
    };

    Ok(Json(UpdateTodoResponse { data: todo }))
}

#[derive(Debug, Serialize)]
pub struct DeleteTodoResponse {
    pub message: &'static str,
}

pub async fn delete_todo(
    Extension(state): Extension<AppState>,
    Extension(user): Extension<UserWithoutPassword>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<DeleteTodoResponse>> {
    match sqlx::query!(
        r#"
        SELECT todo_items.id
        FROM todo_items
        INNER JOIN lists ON lists.id = todo_items.list_id
        WHERE todo_items.id = $1 AND lists.user_id = $2
        "#,
        id,
        user.id // Assuming user.id is the field with the user's ID
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(todo) => todo,
        Err(_) => return Err(ApiError::NotFound),
    };

    let result = sqlx::query!(
        r#"
        DELETE FROM todo_items
        USING lists
        WHERE todo_items.id = $1 AND todo_items.list_id = lists.id AND lists.user_id = $2
        "#,
        id,
        user.id // Assuming user.id is the field with the user's ID
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => Ok(Json(DeleteTodoResponse {
            message: "Todo item successfully deleted",
        })),
        Err(_) => Err(ApiError::InternalServerError),
    }
}
