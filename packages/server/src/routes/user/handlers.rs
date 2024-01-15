use crate::{
    app::AppState,
    error::{ApiError, ApiResult},
    utils::{self, jwt::gen_jwt},
};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RegisterUserResponse {
    user: UserWithoutPassword,
    message: &'static str,
    token: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RegisterUserRequestBody {
    name: Option<String>,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct UserWithoutPassword {
    #[ts(type = "string")]
    pub id: uuid::Uuid,
    pub name: Option<String>,
    pub email: String,
    #[ts(type = "string")]
    pub created_at: chrono::NaiveDateTime,
    #[ts(type = "string")]
    pub updated_at: chrono::NaiveDateTime,
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

    // We can use the "?" operator to convert the error type
    // to ApiError because we implemented From<jsonwebtoken::errors::Error> for ApiError
    // Check the error.rs file
    let token = gen_jwt(user.id)?;

    Ok(Json(RegisterUserResponse {
        user,
        message: "User registered successfully",
        token,
    }))
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LoginUserRequestBody {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LoginUserResponse {
    token: String,
    message: &'static str,
}

pub async fn login_user(
    Extension(state): Extension<AppState>,
    Json(body): Json<LoginUserRequestBody>,
) -> ApiResult<Json<LoginUserResponse>> {
    let db = state.db;

    // Get the user
    let user = match sqlx::query!(
        r#"
        SELECT id, password
        FROM users
        WHERE email = $1
        "#,
        body.email
    )
    .fetch_one(&db)
    .await
    {
        Ok(u) => u,
        Err(_) => {
            return Err(ApiError::NotFound);
        }
    };

    // Verify the password
    let valid = match utils::password::verify_password(&body.password, &user.password) {
        Ok(v) => v,
        Err(_) => {
            return Err(ApiError::InternalServerError);
        }
    };

    if !valid {
        return Err(ApiError::BadRequest);
    }

    // Generate the token
    let token = match utils::jwt::gen_jwt(user.id) {
        Ok(t) => t,
        Err(_) => {
            return Err(ApiError::InternalServerError);
        }
    };

    Ok(Json(LoginUserResponse {
        token,
        message: "success",
    }))
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct GetCurrentUserResponse {
    user: UserWithoutPassword,
    message: &'static str,
}

pub async fn get_current_user(
    Extension(user): Extension<UserWithoutPassword>,
) -> ApiResult<Json<GetCurrentUserResponse>> {
    Ok(Json(GetCurrentUserResponse {
        user,
        message: "success",
    }))
}
