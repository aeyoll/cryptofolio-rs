use rbatis::core::runtime::sync::Arc;
use rbatis::rbatis::Rbatis;

extern crate dotenv;

use askama::Template;

use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::env;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[get("/")]
async fn index(_rb: web::Data<Arc<Rbatis>>, _req: HttpRequest) -> impl Responder {
    // let v = rb.fetch_list::<Cryptocurrency>().await.unwrap();
    let template = IndexTemplate {};

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(template.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let rb = Rbatis::new();
    rb.link(&database_url).await.unwrap();
    let rb = Arc::new(rb);

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .data(rb.to_owned())
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind(&bind)?
    .run()
    .await
}
