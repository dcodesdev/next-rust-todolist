use crate::{
    app::AppState,
    error::{ApiError, ApiResult},
    utils,
};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserResponse {
    user: UserWithoutPassword,
    message: &'static str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserRequestBody {
    name: Option<String>,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWithoutPassword {
    id: uuid::Uuid,
    name: Option<String>,
    email: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

pub async fn register_user(
    Extension(state): Extension<AppState>,
    Json(body): Json<RegisterUserRequestBody>,
) -> ApiResult<Json<RegisterUserResponse>> {
    let db = state.db;

    let hashed = match utils::password::gen_hash(&body.password) {
        Ok(h) => h,
        Err(_) => {
            return Err(ApiError::InternalServerError);
        }
    };

    // Create the user
    let user = match sqlx::query_as!(
        UserWithoutPassword,
        r#"
        INSERT INTO users (name, email, password)
        VALUES ($1, $2, $3)
        RETURNING id, name, email, created_at, updated_at
        "#,
        body.name,
        body.email,
        hashed
    )
    .fetch_one(&db)
    .await
    {
        Ok(u) => u,
        Err(_) => {
            return Err(ApiError::BadRequest);
        }
    };

    Ok(Json(RegisterUserResponse {
        user,
        message: "User registered successfully",
    }))
}
