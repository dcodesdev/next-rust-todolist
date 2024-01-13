use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
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

pub fn decode_jwt(token: &str) -> Option<Uuid> {
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
