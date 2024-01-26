use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn get_posts(pool: web::Data<DbPool>) -> impl Responder {
    use self::schema::posts::dsl::*;

    let conn = pool.get().expect("couldn't get db connection from pool");

    let items = web::block(move || posts.load::<models::Post>(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    HttpResponse::Ok().json(items)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/posts", web::get().to(get_posts))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
