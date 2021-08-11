use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "cryptocurrency_add.html")]
pub struct CryptocurrencyAddTemplate {}
