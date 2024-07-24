use crate::models::user::UserClaim;
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

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct UrlRow {
    pub id: i64,
    pub url: String,
    pub short: String,
    pub user: String,
    pub created: chrono::DateTime<chrono::Utc>,
}

impl Url {
    pub async fn by_short_claim(
        short_code: String,
        claim: UserClaim,
        pool: SqlitePool,
    ) -> Result<UrlRow, Error> {
        let url_row = sqlx::query_as::<_, UrlRow>("SELECT urls.id, urls.url, shorts.short, users.username as user, urls.created FROM urls LEFT JOIN shorts ON shorts.id=urls.short_id LEFT JOIN users ON users.id=urls.user_id WHERE shorts.short=? AND user_id=? LIMIT 1")
        .bind(short_code)
        .bind(claim.id)
        .fetch_one(&pool)
        .await
        .expect("ssssss");

        return Ok(url_row);
    }
    pub async fn by_short(short_code: String, pool: SqlitePool) -> Result<Option<UrlRow>, Error> {
        let url_row = sqlx::query_as::<_, UrlRow>("SELECT urls.id, urls.url, shorts.short, users.username as user, urls.created FROM urls LEFT JOIN shorts ON shorts.id=urls.short_id WHERE shorts.short= ? LIMIT 1")
        .bind(&short_code.as_str())
        .fetch_optional(&pool)
        .await
        .expect("ssssss");

        return Ok(url_row);
    }
    pub async fn by_claim(claim: UserClaim, pool: SqlitePool) -> Result<Vec<UrlRow>, Error> {
        let url_rows = sqlx::query_as::<_,UrlRow>("SELECT urls.id, urls.url, shorts.short, users.username as user, urls.created FROM urls INNER JOIN shorts ON shorts.id=urls.short_id INNER JOIN users ON users.id=urls.user_id WHERE user_id=?")
            .bind(claim.id)
            .fetch_all(&pool)
            .await
            .expect("Error by trying to get all user short links");

        Ok(url_rows)
    }

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
