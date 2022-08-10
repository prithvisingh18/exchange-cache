use std::result::Result;

use futures::executor::block_on;
use reqwest;
use serde_json;

static CURRENCY_EXCHANGE_URL: &str = "https://openexchangerates.org/api/latest.json";
static CURRENCY_EXCHANGE_API_KEY: &str = "b78bf13e29b44fd98d103b450abb7827";

static CRYPTO_EXCHANGE_URL: &str = "https://rest.coinapi.io/v1/assets";
static CRYPTO_EXCHANGE_API_KEY: &str = "37A3014C-3251-401A-8BF5-7E13097DCBAE";

// Print a web page onto stdout
async fn get_currency_exchanges_against_usd() -> Result<serde_json::Value, String> {
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
    return Ok(parsed_values);
}

async fn get_crypto_exchanges_against_usd() -> Result<serde_json::Value, String> {
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

    let parsed_values: serde_json::Value = match serde_json::from_str(&response_body) {
        Ok(pv) => pv,
        Err(_e) => return Err("RESPONSE_PARSING_FAILED".to_string()),
    };
    return Ok(parsed_values);
}

#[tokio::main]
async fn main() {
    let r = block_on(get_currency_exchanges_against_usd());
    println!("r = {:?}", r);

    let r2 = block_on(get_crypto_exchanges_against_usd());
    println!("r2 = {:?}", r2);
}
