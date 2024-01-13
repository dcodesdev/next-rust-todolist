use crate::{
    app::AppState,
    error::{ApiError, ApiResult},
};
use axum::{
    extract::Request, http::header::AUTHORIZATION, middleware::Next, response::Response, Extension,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use sqlx::types::{chrono::NaiveDateTime, Uuid};

#[derive(Debug, Clone)]
pub struct ReqUser {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub async fn auth(
    Extension(state): Extension<AppState>,
    mut req: Request,
    next: Next,
) -> ApiResult<Response> {
    let db = state.db;

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
        ReqUser,
        "SELECT id, name, email, created_at, updated_at FROM users WHERE id = $1",
        &user_id
    )
    .fetch_optional(&db)
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

fn decode_jwt(token: &str) -> Option<Uuid> {
    let key = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");

    #[derive(Deserialize)]
    struct Claims {
        id: String,
    }

    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(key.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    );

    match decoded {
        Err(_) => None,
        Ok(token_data) => {
            let id = token_data.claims.id;

            match Uuid::parse_str(&id) {
                Err(_) => None,
                Ok(id) => Some(id),
            }
        }
    }
}
