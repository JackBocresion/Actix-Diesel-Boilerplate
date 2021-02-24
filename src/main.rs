use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, Error};
use diesel::pg::PgConnection;
pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use diesel::r2d2::{ConnectionManager, Pool};
#[macro_use]
extern crate diesel;

async fn greet(req:HttpRequest) -> impl Responder {
    println!("fi");
    "hello world"
}
use models::ActixUser;
use diesel::RunQueryDsl;

mod models;
mod schema;
use uuid::Uuid;
use crate::models::NewUser;
mod user_helpers;
use user_helpers::create_user;
use argon2::Config;

async fn create(json: web::Json<NewUser>, req:HttpRequest, pool:web::Data<PgPool> ) -> impl Responder {
    use schema::actix_users;
    let conn = pool.get().unwrap();
    let user = create_user(json);
    diesel::insert_into(actix_users::table)
        .values(&user)
        .execute(&conn)
        .unwrap();
    "fsdafs"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let database_url = "postgresql://jackbocresion@localhost";
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/hello", web::get().to(greet))
            .route("create", web::post().to(create))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
/*
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, Error};
use diesel::pg::PgConnection;
pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use diesel::r2d2::{ConnectionManager, Pool};
#[macro_use]
extern crate diesel;

async fn greet(req:HttpRequest) -> impl Responder {
    println!("fi");
    "hello world"
}
use models::ActixUser;
use diesel::RunQueryDsl;

mod models;
mod schema;
use uuid::Uuid;
use crate::models::NewUser;
mod user_helpers;
use user_helpers::create_user;
// #[macro_use] extern crate serde_derive;

async fn create(json: web::Json<NewUser>, req:HttpRequest, pool:web::Data<PgPool> ) -> impl Responder {
    use schema::actix_users;
    let conn = pool.get().unwrap();
    let f = json.username.to_string();
    let g = json.unhashed.to_string();
    let usr = NewUser{username:f, unhashed:g};
    let user = create_user(usr);
    diesel::insert_into(actix_users::table)
        .values(&user)
        .execute(&conn).unwrap();
    "fsdafs"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let database_url = "postgresql://jackbocresion@localhost";
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
        .data(pool.clone())
            .route("/hello", web::get().to(greet))
            .route("create", web::post().to(create))

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
 */