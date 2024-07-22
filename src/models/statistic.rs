use serde::{Deserialize, Serialize};
use sqlx::{Error, SqlitePool};

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Statistic {
    pub id: i64,
    pub url_id: i64,
    pub date: chrono::DateTime<chrono::Utc>,
    pub ip: String,
    pub ua: String,
    pub referer: String,
}

impl Statistic {
    pub async fn create(
        url_id: i64,
        ip: String,
        ua: String,
        referer: String,
        pool: SqlitePool,
    ) -> Result<Statistic, Error> {
        let statistic = sqlx::query_as::<_, Statistic>(
            "INSERT INTO statistics (url_id, ip, ua, referer) VALUES (?, ?, ?, ?) RETURNING id, url_id, date, ip, ua, referer",
        )
        .bind(url_id)
        .bind(ip)
        .bind(ua)
        .bind(referer)
        .fetch_one(&pool)
        .await
        .expect("Error: ");

        Ok(statistic)
    }
}
