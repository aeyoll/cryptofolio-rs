use std::process;

extern crate dotenv;

use failure::Error;

extern crate clogger;

#[macro_use]
extern crate log;

use cryptofolio_coinmarketcap::models::Coinmarketcap;

fn app() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let api_url = "https://pro-api.coinmarketcap.com";
    let api_key = std::env::var("COINMARKETCAP_API_KEY").expect("COINMARKETCAP_API_KEY");

    let cmc = Coinmarketcap::new(api_url, &api_key);
    let quotes = cmc.fetch_listings()?;
    println!("{:?}", quotes);
    println!("{:?}", api_key);

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
