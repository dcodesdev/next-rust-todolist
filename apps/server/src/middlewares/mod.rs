use crate::{
    app::AppState,
    error::{ApiError, ApiResult},
    routes::user::handlers::UserWithoutPassword,
    utils::jwt::decode_jwt,
};
use axum::{
    extract::Request, http::header::AUTHORIZATION, middleware::Next, response::Response, Extension,
};

pub async fn auth(
    Extension(state): Extension<AppState>,
    mut req: Request,
    next: Next,
) -> ApiResult<Response> {
    let db = &*state.db;

    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token_with_bearer = match auth_header {
        None => return Err(ApiError::Unauthorized),
        Some(token) => token,
    };

    let token = token_with_bearer.replace("Bearer ", "");
    let user_id = decode_jwt(&token);

    let user_id = match user_id {
        None => return Err(ApiError::Unauthorized),
        Some(id) => id,
    };

    let user = sqlx::query_as!(
        UserWithoutPassword,
        "SELECT id, name, email, created_at, updated_at FROM users WHERE id = $1",
        &user_id
    )
    .fetch_optional(db)
    .await;

    let user = match user {
        Err(_) => return Err(ApiError::Unauthorized),
        Ok(user) => user,
    };

    match user {
        Some(user) => {
            req.extensions_mut().insert(user);

            return Ok(next.run(req).await);
        }
        None => return Err(ApiError::Unauthorized),
    }
}
