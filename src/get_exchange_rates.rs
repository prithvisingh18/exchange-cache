use std::collections::HashMap;
use std::io::Write;
use std::result::Result;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use futures::executor::block_on;
use reqwest;
use serde_json;

// TODO: Move these values to env variables.
static CURRENCY_EXCHANGE_URL: &str = "https://openexchangerates.org/api/latest.json";
static CURRENCY_EXCHANGE_API_KEY: &str = "b78bf13e29b44fd98d103b450abb7827";

static CRYPTO_EXCHANGE_URL: &str = "https://rest.coinapi.io/v1/assets";
static CRYPTO_EXCHANGE_API_KEY: &str = "37A3014C-3251-401A-8BF5-7E13097DCBAE";

// Print a web page onto stdout
async fn get_currency_exchanges_against_usd() -> Result<HashMap<String, f64>, String> {
    let url = format!(
        "{}?app_id={}&base={}",
        CURRENCY_EXCHANGE_URL, CURRENCY_EXCHANGE_API_KEY, "USD"
    );
    let result_body = reqwest::get(url).await;
    let response_body: String = match result_body {
        Ok(b) => match b.text().await {
            Ok(b_text) => b_text,
            Err(_e) => return Err("HTTP_GET_TEXT_FAILED".to_string()),
        },
        Err(_e) => return Err("HTTP_GET_FAILED".to_string()),
    };

    let parsed_values: serde_json::Value = match serde_json::from_str(&response_body) {
        Ok(pv) => pv,
        Err(_e) => return Err("RESPONSE_PARSING_FAILED".to_string()),
    };

    let exchange_rate_map: HashMap<String, f64> =
        serde_json::from_value(parsed_values["rates"].to_owned()).unwrap();

    return Ok(exchange_rate_map);
}

async fn get_crypto_exchanges_against_usd() -> Result<HashMap<String, f64>, String> {
    let url = format!(
        "{}?filter_asset_id={}",
        CRYPTO_EXCHANGE_URL, "ETH,USDT,BNB,USDC,XRP,LUNA,ADA,SOL,BUSD,AVAX,DOT,DOGE,UST,SHIB"
    );
    let client = reqwest::Client::new();

    let response_body = match client
        .get(url)
        .header("X-CoinAPI-Key", CRYPTO_EXCHANGE_API_KEY)
        .send()
        .await
    {
        Ok(b) => match b.text().await {
            Ok(b_t) => b_t,
            Err(_e) => return Err("HTTP_GET_TEXT_FAILED".to_string()),
        },
        Err(_e) => return Err("HTTP_GET_FAILED".to_string()),
    };
    let mut parsed_values: serde_json::Value = match serde_json::from_str(&response_body) {
        Ok(pv) => pv,
        Err(_e) => return Err("RESPONSE_PARSING_FAILED".to_string()),
    };

    let mut counter = 0;
    let mut m = HashMap::new();
    let mut crypto_rate_map = loop {
        if counter == parsed_values.as_array().unwrap().len() {
            break m;
        } else {
            m.insert(
                parsed_values[counter]["asset_id"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                1.0 / parsed_values[counter]["price_usd"]
                    .as_f64()
                    .unwrap()
                    .to_owned(),
            );
            counter += 1;
        }
    };

    return Ok(crypto_rate_map);
}

async fn get_exchange_rates() -> HashMap<String, f64> {
    let mut r = get_currency_exchanges_against_usd().await.unwrap();
    let r2 = get_crypto_exchanges_against_usd().await.unwrap();
    r.extend(r2);
    return r;
}

async fn load_exchange_rates_from_file_or() -> Result<HashMap<String, f64>, String> {
    let exchange_rates_path = Path::new("/tmp/exchange_rates_usd.json");
    if exchange_rates_path.exists() {
        println!("Path exists.");
        let mut exchange_rate_file = match File::open(exchange_rates_path) {
            Ok(file) => file,
            Err(error) => return Err("Error opening file.".to_string()),
        };
        let mut exchange_rate_string = String::new();
        match exchange_rate_file.read_to_string(&mut exchange_rate_string) {
            Ok(_) => {}
            Err(error) => return Err("Error opening exchange rates.".to_string()),
        }
        let exchange_rates_pv = match serde_json::from_str(&exchange_rate_string) {
            Ok(pv) => pv,
            Err(_e) => return Err("STRING_PARSING_FAILED".to_string()),
        };
        return Ok(exchange_rates_pv);
    } else {
        println!("Exchange rate file not found.");
        let mut exchange_rate_file = match File::create(exchange_rates_path) {
            Ok(file) => file,
            Err(error) => return Err("Error creating file.".to_string()),
        };
        let exchange_rates = get_exchange_rates().await;
        let exchange_rate_string = match serde_json::to_string(&exchange_rates) {
            Ok(string) => string,
            Err(error) => return Err("Error creating  map string.".to_string()),
        };
        exchange_rate_file.write_all(exchange_rate_string.as_bytes());
        match exchange_rate_file.sync_all() {
            Ok(_) => {}
            Err(error) => return Err("error syncing".to_string()),
        };
        return Ok(exchange_rates);
    }
}

async fn convert_currency(
    from_currency: String,
    to_currency: String,
    value: f64,
) -> Result<f64, String> {
    let r = match load_exchange_rates_from_file_or().await {
        Ok(r) => r,
        Err(error) => return Err("error geting exchange rates".to_string()),
    };
    for (asset, rate) in &r {
        println!("{asset}: {rate}");
    }

    /*
        usd -> value / r[from_currency]
        return usd * r[to_currency]
    */

    println!("{} {} to {}", from_currency, value, to_currency);

    let mut converted_currency: f64 = (value / r[&from_currency]);
    converted_currency = converted_currency * r[&to_currency];

    Ok(converted_currency)
}

#[tokio::main]
async fn main() {
    let c = convert_currency("INR".to_string(), "USD".to_string(), 300.0)
        .await
        .unwrap();
    println!("{}", c);

    // for (asset, rate) in &r2 {
    //     println!("{asset}: \"{rate}\"");
    // }
    // println!("r2 = {:?}", r2);

    // block_on(load_exchange_rates_from_file());

    return;
}
