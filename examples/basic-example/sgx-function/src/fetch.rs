use switchboard_utils::Error;
use weather_station;

use std::time::{SystemTime, UNIX_EPOCH};
use switchboard_utils::task::http_task;

use futures::stream::FuturesUnordered;
use std::time::Duration;
use tokio::time::timeout;

pub async fn fetch_weather_report() -> Result<Vec<weather_station::WeatherReport>, Error> {
    // Might need to worry about rate limiting
    let meteo_sources = vec![
        "https://api.open-meteo.com/v1/forecast?latitude=40.7831&longitude=-73.9712&current_weather=true&temperature_unit=fahrenheit",
        "https://api.open-meteo.com/v1/forecast?latitude=51.5072&longitude=-0.1276&current_weather=true&temperature_unit=fahrenheit",
        "https://api.open-meteo.com/v1/forecast?latitude=22.3193&longitude=114.1694&current_weather=true&temperature_unit=fahrenheit",
        "https://api.open-meteo.com/v1/forecast?latitude=37.7749&longitude=-122.4194&current_weather=true&temperature_unit=fahrenheit"
    ];

    let tasks = meteo_sources
        .into_iter()
        .map(|url| {
            tokio::spawn(async move {
                let result = http_task(url, Some("$.current_weather.temperature")).await;
                return result.unwrap_or_default();
            })
        })
        .collect::<FuturesUnordered<_>>();

    let result = timeout(Duration::from_secs(5), futures::future::join_all(tasks))
        .await
        .expect("failed to fetch all sources");
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    Ok(result
        .into_iter()
        .map(|r| weather_station::WeatherReport {
            temperature: r.unwrap_or_default().as_f64().unwrap_or_default(),
            timestamp: since_the_epoch.as_secs(),
            weathercode: 0,
            windspeed: "f",
        })
        .collect())
}
