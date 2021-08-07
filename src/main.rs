#[macro_use]
extern crate rbatis;

use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::core::runtime::sync::Arc;

extern crate dotenv;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, Result, get, middleware, web};
use std::env;

use crate::models::Cryptocurrency;

mod models;

#[get("/")]
async fn index(rb: web::Data<Arc<Rbatis>>, req: HttpRequest) -> impl Responder {
    let v = rb.fetch_list::<Cryptocurrency>().await.unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("Hello"))
}

#[get("/api/update")]
async fn api_update(rb: web::Data<Arc<Rbatis>>) -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let rb = Rbatis::new();
    rb.link(&connspec).await.unwrap();
    let rb = Arc:: new(rb);

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .data(rb.to_owned())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(api_update)
    })
    .bind(&bind)?
    .run()
    .await
}
