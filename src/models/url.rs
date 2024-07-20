use serde::{Deserialize, Serialize};
use sqlx::{Error, SqlitePool};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Short {
    pub id: i64,
    pub short: String,
}

#[derive(Serialize, Deserialize)]
pub struct UrlRequest {
    pub url: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Url {
    pub id: i64,
    pub url: String,
    pub short_id: i64,
    pub user_id: i64,
    pub created: chrono::DateTime<chrono::Utc>,
}
impl Url {
    pub async fn create(url: String, uid: i64, pool: SqlitePool) -> Result<Url, Error> {
        if let Ok(short) = Short::get(pool.clone()).await {
            let url = sqlx::query_as::<_, Url>("INSERT INTO urls (url,short_id,user_id) VALUES (?,?,?) RETURNING id, url, short_id, user_id, created")
                .bind(url)
                .bind(short.id)
                .bind(uid)
                .fetch_one(&pool)
                .await
                .expect("Error by trying to create new url");

            return Ok(url);
        }
        Err(sqlx::Error::RowNotFound)
    }
}

impl Short {
    pub async fn get(pool: SqlitePool) -> Result<Short, Error> {
        let short = sqlx::query_as::<_, Short>("SELECT id, short FROM shorts WHERE id NOT IN (SELECT DISTINCT short_id FROM urls) LIMIT 1")
            .fetch_one(&pool)
            .await
            .expect("Error fetching data");

        Ok(short)
    }
}
