use askama::Template;
use rbatis::core::runtime::sync::Arc;
use rbatis::rbatis::Rbatis;
use serde::{Deserialize, Serialize};

extern crate dotenv;

mod templates;
use crate::templates::{CryptocurrencyAddTemplate, IndexTemplate};

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use std::env;

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
            .wrap(middleware::Logger::default())
            .data(rb.to_owned())
            .configure(app_config)
    })
    .bind(&bind)?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/cryptocurrency/add").route(web::post().to(cryptocurency_add))),
    );
}

async fn index(_rb: web::Data<Arc<Rbatis>>, _req: HttpRequest) -> Result<HttpResponse> {
    // let v = rb.fetch_list::<Cryptocurrency>().await.unwrap();
    let template = IndexTemplate {};

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(template.render().unwrap()))
}

#[derive(Serialize, Deserialize)]
pub struct CryptocurrencyAddParams {
    name: String,
    spent: f64,
}

async fn cryptocurency_add(
    rb: web::Data<Arc<Rbatis>>,
    params: web::Form<CryptocurrencyAddParams>,
) -> Result<HttpResponse> {
    let template = CryptocurrencyAddTemplate {};

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(template.render().unwrap()))
}
