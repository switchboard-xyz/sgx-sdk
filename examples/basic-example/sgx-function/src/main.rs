use solana_sdk::pubkey::Pubkey;
use std::ops::Deref;
use std::sync::Arc;

use clokwerk::{AsyncScheduler, TimeUnits};
use std::thread;
use std::time::Duration;

pub mod config;
pub mod error;
pub mod fetch;
pub mod switchboard;

pub use config::*;
pub use error::*;
pub use fetch::*;
pub use switchboard::*;

pub type Keypair = Arc<anchor_client::solana_sdk::signer::keypair::Keypair>;
pub type AnchorClient = Arc<anchor_client::Client<Keypair>>;
pub type AnchorProgram = Arc<anchor_client::Program<Keypair>>;

#[tokio::main(worker_threads = 12)]
async fn main() -> Result<(), Error> {
    // setup logging
    simple_logger::init_with_env().unwrap();

    let config = Config::new()?;

    let weather_program: AnchorProgram = Arc::new(config.client.program(weather_station::id()));
    let switchboard_program: AnchorProgram = Arc::new(config.client.program(switchboard::PID));

    let quotekp: Keypair =
        switchboard::run_init_quote(config.client.clone(), config.payer.clone()).await;

    let (station, _station_bump) =
        Pubkey::find_program_address(&[weather_station::WEATHER_SEED], &weather_station::id());

    let station_account: weather_station::WeatherStation = weather_program
        .account(station)
        .expect("station account should already be initialized");

    log::info!("station: {}", station);
    log::info!("authority: {}", station_account.authority);
    log::info!("schema: {}", station_account.schema);

    let mut scheduler = AsyncScheduler::with_tz(chrono::Utc);

    // update the weather reports every 5 min
    scheduler
        .every(5.minutes())
        .run(move || run_save_report(weather_program.clone(), station, quotekp.clone()));

    // heartbeat every 60 seconds
    scheduler
        .every(60.seconds())
        .run(move || switchboard::run_heartbeat(switchboard_program.clone()));

    // TODO: Add quote publish task every 24 hrs ?? (Might not be needed)

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(10));
    }
}

pub async fn run_save_report(program: AnchorProgram, station: Pubkey, quotekp: Keypair) {
    // fetch reports
    let report_result = fetch_weather_report().await;
    if report_result.is_err() {
        // log and exit
        log::warn!("Failed to fetch weather reports");
        return;
    }

    // save reports
    match program
        .request()
        .signer(quotekp.deref())
        .accounts(weather_station::accounts::Report { station })
        .args(weather_station::instruction::Report {
            params: weather_station::ReportParams {
                reports: report_result.unwrap(),
            },
        })
        .send()
    {
        Err(_) => log::warn!("Failed to save weather reports"),
        Ok(tx) => log::info!("save_reports signature: {}", tx),
    }
}
