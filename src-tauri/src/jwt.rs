use std::env;

use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    user_type: String,
}

pub async fn create_jwt() -> Result<String, String> {
    dotenv().ok();
    let secret = env::var("JWT_SECRET").map_err(|_| "JWT_SECRET non trouvé".to_string())?;
    let claims = Claims {
        user_type: "admin".to_string(),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();
    Ok(token)
}

