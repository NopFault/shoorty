use crate::models::user::UserClaim;
use chrono::{Datelike, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use nopnv::Env;
use actix_web::HttpRequest;

fn get_secret_key() -> String {
    let env = Env::from_file(".env").expect("Failed to read .env file");

    let base_secret: String = env
        .get("SECRET_KEY")
        .unwrap_or(&String::from("testas"))
        .to_string();
    let today = Utc::now();
    format!("{}-{}-{}", base_secret, today.year(), today.ordinal())
}

pub fn create_jwt(username: &str, id: i64) -> String {
    let secret: String = get_secret_key();
    let expiration = 3600;

    let claims = UserClaim {
        username: username.to_owned(),
        id,
        exp: (Utc::now() + chrono::Duration::seconds(expiration)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn validate_jwt(token: &str) -> Result<UserClaim, jsonwebtoken::errors::Error> {
    let secret = get_secret_key();
    decode::<UserClaim>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

pub fn get_claim_from(req: &HttpRequest) -> Option<UserClaim> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                if let Ok(user_claim) = validate_jwt(token) {
                    return Some(user_claim);
                }
            }
        }
    }
    None
}
