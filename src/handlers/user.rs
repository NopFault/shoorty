use crate::auth::get_claim_from;
use crate::models::user::{User, UserLoginRequest};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use sqlx::SqlitePool;

pub async fn user_login(
    req: web::Json<UserLoginRequest>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    let credentials = UserLoginRequest {
        username: req.username.to_string(),
        password: req.password.to_string(),
    };

    if let Ok(token) = User::get_by_creds(credentials, pool.get_ref().clone()).await {
        return Ok(HttpResponse::Ok().json(token));
    }
    Ok(HttpResponse::InternalServerError().body("Ne tavo kiskis ne tu ir kiskis"))
}

pub async fn user_register(
    new_user: web::Json<UserLoginRequest>,
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    let newuser = UserLoginRequest {
        username: new_user.username.to_string(),
        password: new_user.password.to_string(),
    };
    if let Ok(user) = User::create(newuser, pool.get_ref().clone()).await {
        return Ok(HttpResponse::Ok().json(user));
    }
    Ok(HttpResponse::InternalServerError().body("Ne tavo kiskis ne tu ir kiskis"))
}

pub async fn get(req: HttpRequest, pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    if let Some(userclaim) = get_claim_from(&req) {
        if let Ok(user) = User::get(userclaim.id, pool.get_ref().clone()).await {
            return Ok(HttpResponse::Ok().json(user));
        }
    }
    Ok(HttpResponse::InternalServerError().body("Ne tavo kiskis ne tu ir kiskis"))
}

pub async fn get_all(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    if let Ok(users) = User::list(pool.get_ref().clone()).await {
        return Ok(HttpResponse::Ok().json(users));
    }

    Ok(HttpResponse::InternalServerError().body("Ne tavo kiskis ne tu ir kiskis"))
}
