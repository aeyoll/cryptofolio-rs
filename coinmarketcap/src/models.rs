use failure::Error;
use serde::Deserialize;

use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Cryptocurrency {
    pub id: i32,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub quote: HashMap<String, Quote>,
}

#[derive(Deserialize, Debug)]
pub struct Quote {
    pub price: f64,
    pub percent_change_1h: Option<f64>,
    pub percent_change_24h: Option<f64>,
    pub percent_change_7d: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct CryptocurrencyListings {
    data: Vec<Cryptocurrency>,
}

pub struct Coinmarketcap {
    api_url: String,
    api_key: String,
}

impl Coinmarketcap {
    pub fn new(api_url: &str, api_key: &str) -> Coinmarketcap {
        Coinmarketcap {
            api_url: api_url.into(),
            api_key: api_key.into(),
        }
    }

    pub fn fetch_listings(&self) -> Result<CryptocurrencyListings, Error> {
        let url = format!("{}/{}", self.api_url, "v1/cryptocurrency/listings/latest");
        let response = ureq::get(&url)
            .set("X-CMC_PRO_API_KEY", &self.api_key)
            .query("convert", "EUR")
            .query("limit", "200")
            .call()?
            .into_string()?;

        Ok(serde_json::from_str::<CryptocurrencyListings>(&response)?)
    }
}
