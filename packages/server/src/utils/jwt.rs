use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    id: Uuid,
}

pub fn gen_jwt(id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let claims = Claims {
        // Expires in 7 days
        exp: (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize,
        id,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    );

    token
}
