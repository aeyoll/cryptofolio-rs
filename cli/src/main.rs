use std::process;
use cryptofolio_core::models::CmcCryptocurrency;

extern crate dotenv;

use failure::Error;

extern crate clogger;

#[macro_use]
extern crate log;

fn app() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest";
    let api_key = std::env::var("COINMARKETCAP_API_KEY").expect("COINMARKETCAP_API_KEY");

    let _body: Vec<CmcCryptocurrency> = ureq::get(&url)
        .set("Accepts", "application/json")
        .set("X-CMC_PRO_API_KEY", &api_key)
        .send_json(ureq::json!({
            "convert": "EUR",
            "limit": 200,
        }))?
        .into_json()?;

    Ok(())
}

fn main() {
    clogger::init();

    process::exit(match app() {
        Ok(_) => 0,
        Err(err) => {
            error!("{}", err.to_string());
            1
        }
    });
}
