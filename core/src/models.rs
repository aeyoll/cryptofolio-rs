use serde::{Deserialize, Serialize};

#[crud_table]
#[derive(Debug, Serialize, Deserialize)]
pub struct Cryptocurrency {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub price: Option<f64>,
    pub spent: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CmcQuote {
    price: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CmcCryptocurrency {
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub quote: Vec<CmcQuote>,
}
