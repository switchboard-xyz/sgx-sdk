// use anchor_client::solana_sdk::signer::keypair::Keypair;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::read_keypair_file;
use std::ops::Deref;
use std::sync::Arc;

use clokwerk::{AsyncScheduler, TimeUnits};
use std::thread;
use std::time::Duration;

pub mod fetch;
pub mod switchboard;

pub use fetch::*;
pub use switchboard::*;

pub type Keypair = Arc<anchor_client::solana_sdk::signer::keypair::Keypair>;
pub type AnchorClient = Arc<anchor_client::Client<Keypair>>;
pub type AnchorProgram = Arc<anchor_client::Program<Keypair>>;

#[tokio::main(worker_threads = 12)]
async fn main() {
    // setup logging
    simple_logger::init_with_env().unwrap();

    let url = std::env::var("RPC_URL").unwrap();
    let wss_url = url.replace("https://", "wss://");
    let cluster = anchor_client::Cluster::Custom(url.clone(), wss_url.clone());
    let payer: Keypair = Arc::new(read_keypair_file(std::env::var("PAYER").unwrap()).unwrap());
    let client: AnchorClient = Arc::new(anchor_client::Client::new_with_options(
        cluster.clone(),
        payer.clone(),
        CommitmentConfig::processed(),
    ));
    let weather_program: AnchorProgram = Arc::new(client.program(weather_station::id()));
    let switchboard_program: AnchorProgram = Arc::new(client.program(switchboard::PID));

    let quotekp: Keypair = switchboard::run_init_quote(client.clone(), payer.clone()).await;

    let (station, _station_bump) =
        Pubkey::find_program_address(&[weather_station::WEATHER_SEED], &weather_station::id());

    let station_account: weather_station::WeatherStation = weather_program
        .account(station)
        .expect("station account should already be initialized");

    log::info!("station: {}", station);
    log::info!("authority: {}", station_account.authority);
    log::info!("schema: {}", station_account.schema);
    // log::info!("last_updated: {}", station_account.last_updated);

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
