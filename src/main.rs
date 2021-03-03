use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, error, get, post};
use diesel::pg::PgConnection;
pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use diesel::r2d2::{ConnectionManager, Pool};
#[macro_use]
extern crate diesel;
#[get("/hello")]
async fn greet(req:HttpRequest) -> impl Responder {
    "hello world"
}
use models::ActixUser;
use diesel::{RunQueryDsl, QueryDsl};

mod models;
mod schema;
use uuid::Uuid;
use crate::models::NewUser;
mod user_helpers;
use argon2::Config;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

#[post("/create")]
async fn create_user(json: web::Json<NewUser>, req:HttpRequest, pool:web::Data<PgPool> ) -> Result<web::Json<ActixUser>, HttpResponseBuilder> {
    use schema::actix_users;
    let conn = pool.get().unwrap();
    let user = ActixUser::new(json);
    let inserted_user = diesel::insert_into(actix_users::table)
        .values(&user)
        .get_result::<ActixUser>(&conn);
    match inserted_user {
        Ok(n) => Ok(web::Json(n)),
        _ => Err(HttpResponse::Forbidden())
    }

}

use actix_web::http::{StatusCode};
#[get("/getall")]
async fn get_users(req:HttpRequest, pool:web::Data<PgPool>) -> Result<web::Json<Vec<ActixUser>>, HttpResponseBuilder> {
    use schema::actix_users::dsl::*;
    let conn = pool.get().unwrap();
    let users = actix_users
        .load::<ActixUser>(&conn);
    match users {
        Ok(v) => {
            match v.len() {
                0=> Err(HttpResponse::NoContent()),
                _ => Ok(web::Json(v))
            }
        },
        _ => Err(HttpResponse::InternalServerError())
    }
}
//"f!".with_status(StatusCode::FORBIDDEN) with impl Responder
use std::io::Error;
use actix_web::web::Json;
use std::option;
use serde::{Serialize, Serializer};
use actix_web::dev::HttpResponseBuilder;


#[get("/getone/{username}")]
async fn get_one_user(_req:HttpRequest, pool:web::Data<PgPool>, usr:web::Path<String>) -> Result<web::Json<ActixUser>, HttpResponseBuilder> {
    use schema::actix_users::dsl::*;
    use self::diesel::prelude::*;
    println!("{}",usr);
    let conn = pool.get().unwrap();
    let user = actix_users
        .filter(username.eq(usr.to_string()))
        .first::<ActixUser>(&conn);//.expect("f");
    match user {
        Ok(u) => {
            Ok(web::Json(u))
        },
        _ => {
            Err(HttpResponse::BadRequest())
        }
    }
    // web::Json(ret)
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(greet)
            .service(create_user)
            .service(get_users)
            .service(get_one_user)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}