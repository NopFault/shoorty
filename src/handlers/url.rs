use crate::auth::get_claim_from;
use crate::models::statistic::Statistic;
use crate::models::url::{Url, UrlRequest};
use crate::parsers::http::RequestParser;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use sqlx::SqlitePool;

pub async fn get_all(req: HttpRequest, pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    if let Some(userclaim) = get_claim_from(&req) {
        if let Ok(urls) = Url::by_claim(userclaim, pool.get_ref().clone()).await {
            return Ok(HttpResponse::Ok().json(urls));
        }
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
pub async fn get(
    req: HttpRequest,
    short_code: web::Path<String>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    if let Some(userclaim) = get_claim_from(&req) {
        if let Ok(url) =
            Url::by_short_claim(short_code.to_string(), userclaim, pool.get_ref().clone()).await
        {
            return Ok(HttpResponse::Ok().json(url));
        }
    }
    Ok(HttpResponse::InternalServerError().body("Ne tavo kiskis ne tu ir kiskis"))
}
pub async fn redirect(
    req: HttpRequest,
    short_code: web::Path<String>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    let result = Url::by_short(short_code.to_string(), pool.get_ref().clone()).await;
    match result {
        Ok(Some(url)) => {
            let parser = RequestParser::new(&req);
            let _stats = Statistic::create(
                url.id,
                String::from(parser.get_ip()),
                String::from(parser.get_ua()),
                String::from(parser.get_referer()),
                pool.get_ref().clone(),
            )
            .await;
            Ok(HttpResponse::Found()
                .insert_header(("Location", url.url))
                .finish())
        }
        Ok(None) => Ok(HttpResponse::Found()
            .insert_header(("Location", "https://tnyuri.com"))
            .finish()),
        Err(_) => Ok(HttpResponse::Found()
            .insert_header(("Location", "https://tnyuri.com"))
            .finish()),
    }
}
