use askama::Template;
use cryptofolio_core::models::Cryptocurrency;
use rbatis::{core::runtime::sync::Arc, crud::CRUD};
use rbatis::rbatis::Rbatis;
use serde::{Deserialize, Serialize};

extern crate dotenv;

mod templates;
use crate::templates::{CryptocurrencyAddTemplate, IndexTemplate};

use actix_web::{
    http::header::ContentType, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result,
};
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
            .service(
                web::scope("/cryptocurrency")
                    .route("/add", web::get().to(cryptocurency_add))
                    .route("/create", web::post().to(cryptocurency_create)),
            ),
    );
}

async fn index(_rb: web::Data<Arc<Rbatis>>, _req: HttpRequest) -> Result<HttpResponse> {
    // let v = rb.fetch_list::<Cryptocurrency>().await.unwrap();
    let template = IndexTemplate {};

    Ok(HttpResponse::Ok()
        .set(ContentType::html())
        .body(template.render().unwrap()))
}

#[derive(Serialize, Deserialize)]
pub struct CryptocurrencyAddParams {
    name: String,
    spent: f64,
}

async fn cryptocurency_add() -> Result<HttpResponse> {
    let template = CryptocurrencyAddTemplate {};

    Ok(HttpResponse::Ok()
        .set(ContentType::html())
        .body(template.render().unwrap()))
}

async fn cryptocurency_create(
    rb: web::Data<Arc<Rbatis>>,
    params: web::Form<CryptocurrencyAddParams>,
) -> Result<HttpResponse> {
    let template = CryptocurrencyAddTemplate {};

    println!("Your name is {}", params.name);
    println!("Your spent is {}", params.spent);

    let cc = Cryptocurrency {
        id: None,
        name: Some(params.name.to_owned()),
        spent: Some(params.spent),
        price: Some(0.0),
    };

    rb.save(&cc, &[]).await;

    Ok(HttpResponse::Ok()
        .set(ContentType::html())
        .body(template.render().unwrap()))
}
