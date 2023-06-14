// Note: Binance API requires a non-US IP address

use serde::Deserialize;
pub use switchboard_solana::prelude::*;

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct Ticker {
    pub symbol: String,
    pub weightedAvgPrice: String,
    pub lastPrice: String,
    pub volume: String,
}

pub async fn fetch_prices(
    symbols: Vec<&str>,
) -> std::result::Result<Vec<Ticker>, SwitchboardClientError> {
    let tickers = reqwest::get(format!(
        "https://api.binance.com/api/v3/ticker?symbols=[{}]&windowSize=1h",
        symbols
            .iter()
            .map(|x| format!("\"{}\"", x))
            .collect::<Vec<String>>()
            .join(",")
    ))
    .await
    .unwrap()
    .json::<Vec<Ticker>>()
    .await
    .unwrap();

    Ok(tickers)
}
