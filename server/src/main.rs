extern crate tera;
use rbatis::crud::CRUD;
use tera::Context;

use rbatis::rbatis::Rbatis;
use rbatis::core::runtime::sync::Arc;
use serde::{Deserialize, Serialize};

extern crate dotenv;
use std::env;

use actix_web::{middleware, App, HttpRequest, HttpResponse, HttpServer};
use actix_web::web::{get, post, resource, scope, ServiceConfig, Data, Form};

use crate::request::flash::FlashMessages;
use crate::request::request::Render;

use cryptofolio_core::models::Cryptocurrency;

pub mod error;
pub mod request;
pub mod templates;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let template_store = crate::templates::load();
    let templates = template_store.templates.clone();

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
            .app_data(templates.clone())
            .configure(app_config)
    })
    .bind(&bind)?
    .run()
    .await
}

fn app_config(config: &mut ServiceConfig) {
    config
        .service(resource("/").route(get().to(index)))
        .service(scope("/cryptocurrency")
            .service(resource("/add")
                .route(get().to(cryptocurency_add))
                .route(post().to(cryptocurency_create)),
            )
        )
    ;
}

pub async fn index(request: HttpRequest, rb: Data<Arc<Rbatis>>) -> Result<HttpResponse> {
    let cryptocurrencies = rb.fetch_list::<Cryptocurrency>().await.unwrap();
    request.render(200, "index.html", {
        let mut context = Context::new();
        context.insert("cryptocurrencies", &cryptocurrencies);
        context
    })
}

#[derive(Serialize, Deserialize)]
pub struct CryptocurrencyAddParams {
    name: String,
    balance: f64,
    spent: f64,
}

pub async fn cryptocurency_add(request: HttpRequest) -> Result<HttpResponse> {
    request.render(200, "cryptocurrency_index.html", {
        let context = Context::new();
        context
    })
}

pub async fn cryptocurency_create(
    request: HttpRequest,
    rb: Data<Arc<Rbatis>>,
    params: Form<CryptocurrencyAddParams>,
) -> Result<HttpResponse> {
    let cc = Cryptocurrency {
        id: None,
        name: Some(params.name.to_owned()),
        balance: Some(params.balance.to_owned()),
        spent: Some(params.spent),
        price: Some(0.0),
    };

    let _ = rb.save(&cc, &[]).await;

    request.flash("Success", "Cryptocurrency succesfully added.")?;
    request.redirect("/")
}
