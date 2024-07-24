use actix_web::{web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use nopnv::Env;
use sqlx::sqlite::SqlitePoolOptions;

mod auth;
mod handlers;
mod middlewares;
mod models;
mod parsers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = Env::from_file(".env").expect("Failed to read .env file");

    let pool = SqlitePoolOptions::new()
        .connect(env.get("DATABASE").unwrap_or(&String::from("db.sqlite")))
        .await
        .expect("cannot get the pool from sqlite");

    // TODO:
    // Refactor into normal migration
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS urls (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            url TEXT NOT NULL,
            short_id INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
            created DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS shorts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            short TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS statistics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            url_id INTEGER NOT NULL,
            date DATETIME DEFAULT CURRENT_TIMESTAMP,
            ip TEXT,
            ua TEXT,
            referer TEXT
        );
        
        INSERT OR REPLACE INTO users (id, username,password) VALUES (1,'nopfault','5fd924625f6ab16a19cc9807c7c506ae1813490e4ba675f843d5a10e0baacdb8');
        
        INSERT OR REPLACE INTO shorts (id, short) VALUES (1,'lucky'); 
        INSERT OR REPLACE INTO shorts (id, short) VALUES (2,'test');
        INSERT OR REPLACE INTO shorts (id, short) VALUES (3,'rusty'); 
        INSERT OR REPLACE INTO shorts (id, short) VALUES (4,'crab'); 
        INSERT OR REPLACE INTO shorts (id, short) VALUES (5,'dev'); 
        INSERT OR REPLACE INTO shorts (id, short) VALUES (6,'short'); 
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create table.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/login", web::post().to(handlers::user::user_login))
            .route(
                "/register",
                web::post()
                    .to(handlers::user::user_register)
                    .wrap(from_fn(middlewares::auth::root_gate)),
            )
            .route(
                "/urls",
                web::get()
                    .to(handlers::url::get_all)
                    .wrap(from_fn(middlewares::auth::auth_gate)),
            )
            .route(
                "/url/add",
                web::post()
                    .to(handlers::url::create)
                    .wrap(from_fn(middlewares::auth::auth_gate)),
            )
            .route(
                "/url/{short_code}",
                web::get()
                    .to(handlers::url::get)
                    .wrap(from_fn(middlewares::auth::auth_gate)),
            )
            .route(
                "/admin/users",
                web::get()
                    .to(handlers::user::get_all)
                    .wrap(from_fn(middlewares::auth::root_gate)),
            )
            .route("/{short_code}", web::get().to(handlers::url::redirect))
    })
    .bind(
        env.get("HOSTNAME")
            .unwrap_or(&String::from("localhost:8181")),
    )?
    .run()
    .await
}
