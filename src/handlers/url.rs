use crate::auth::get_claim_from;
use crate::models::url::{Short, Url, UrlRequest};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use sqlx::SqlitePool;

pub async fn short(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    if let Ok(short) = Short::get(pool.get_ref().clone()).await {
        return Ok(HttpResponse::Ok().body(short.short));
    }

    Ok(HttpResponse::InternalServerError().body("Ne tavo kiskis ne tu ir kiskis"))
}

pub async fn create(
    req: HttpRequest,
    urlreq: web::Json<UrlRequest>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    if let Some(userclaim) = get_claim_from(&req) {
        if let Ok(newurl) =
            Url::create(urlreq.url.to_string(), userclaim.id, pool.get_ref().clone()).await
        {
            return Ok(HttpResponse::Ok().json(newurl));
        }
    }
    Ok(HttpResponse::InternalServerError().body("Ne tavo kiskis ne tu ir kiskis"))
}
