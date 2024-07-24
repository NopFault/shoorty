use crate::auth::create_jwt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{Error, SqlitePool};

#[derive(Deserialize, Debug)]
pub struct UserLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct UserLoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaim {
    pub username: String,
    pub id: i64,
    pub exp: usize,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn get(
        credentials: UserLoginRequest,
        pool: SqlitePool,
    ) -> Result<UserLoginResponse, Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password FROM users WHERE username = ?",
        )
        .bind(&credentials.username)
        .fetch_one(&pool)
        .await
        .expect("Error: ");

        if User::hash_password(String::from(&credentials.password)) == String::from(&user.password)
        {
            let token = create_jwt(&user.username, user.id);
            return Ok(UserLoginResponse { token });
        }
        Ok(UserLoginResponse {
            token: "".to_string(),
        })
    }

    pub async fn create(credentials: UserLoginRequest, pool: SqlitePool) -> Result<User, Error> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, password) VALUES (?, ?) RETURNING id, username, password",
        )
        .bind(&credentials.username)
        .bind(User::hash_password(credentials.password.clone()))
        .fetch_one(&pool)
        .await
        .expect("Error: ");

        Ok(user)
    }

    pub async fn list(pool: SqlitePool) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&pool)
            .await
            .expect("Error: ");

        Ok(users)
    }

    fn hash_password(pass: String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(pass);
        let result = hasher.finalize();
        hex::encode(result)
    }
}
