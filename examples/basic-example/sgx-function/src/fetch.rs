use switchboard_utils::Error;
use weather_station;

use switchboard_utils::task::http_task;

use std::time::Duration;
use tokio::{join, time::timeout};

pub async fn fetch_weather_report() -> Result<Vec<weather_station::WeatherReport>, Error> {
    let timeout_duration = Duration::from_secs(5);

    let result = timeout(
        timeout_duration,
        join!(
            // New York
            http_task("https://api.open-meteo.com/v1/forecast?latitude=40.7831&longitude=-73.9712&current_weather=true&temperature_unit=fahrenheit", "$.current_weather.temperature"),
            // London
            http_task("https://api.open-meteo.com/v1/forecast?latitude=51.5072&longitude=-0.1276&current_weather=true&temperature_unit=fahrenheit", "$.current_weather.temperature")
            // Hong Kong
            http_task("https://api.open-meteo.com/v1/forecast?latitude=22.3193&longitude=114.1694&current_weather=true&temperature_unit=fahrenheit", "$.current_weather.temperature")
            // San Fran
            http_task("https://api.open-meteo.com/v1/forecast?latitude=37.7749&longitude=-122.4194&current_weather=true&temperature_unit=fahrenheit", "$.current_weather.temperature")
        ),
    ).await;

    Ok(vec![])
}
