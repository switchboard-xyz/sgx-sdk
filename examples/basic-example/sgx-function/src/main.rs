use anchor_client::solana_sdk::signer::keypair::Keypair;
use anchor_client::{Client, ClientError};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signer::keypair::read_keypair_file;
use solana_sdk::signer::Signer;
use solana_sdk::{pubkey, pubkey::Pubkey};
use std::ops::Deref;
use std::sync::Arc;

use clokwerk::{AsyncScheduler, TimeUnits};
use std::thread;
use std::time::Duration;

pub mod fetch;
pub mod switchboard;

pub use fetch::*;
pub use switchboard::*;

const WEATHER_STATION_PID: Pubkey = pubkey!("BZ7ambpq5TN49KczyL7QEfyr2xmnz1ubAHqdSC5ywd5f");
const SWITCHBOARD_PID: Pubkey = pubkey!("SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f");

const VERIFIER_QUEUE: Pubkey = pubkey!("4AnQSCo6YJmmYVq2BUHx5vfxoqktpBmTbDh1NurxShgN");

type AnchorClient = anchor_client::Client<Arc<Keypair>>;

#[tokio::main(worker_threads = 12)]
async fn main() {
    // setup logging
    simple_logger::init_with_env().unwrap();

    let url = std::env::var("RPC_URL").unwrap();
    let wss_url = url.replace("https://", "wss://");
    let cluster = anchor_client::Cluster::Custom(url.clone(), wss_url.clone());
    let payer = Arc::new(read_keypair_file(std::env::var("PAYER").unwrap()).unwrap());
    // let payer_pubkey = payer.try_pubkey().expect("failed to get payer pubkey");
    let anchor_client = anchor_client::Client::new_with_options(
        cluster.clone(),
        payer.clone(),
        CommitmentConfig::processed(),
    );

    let quotekp = solana_init_quote(&anchor_client, payer.clone()).await;

    let (station, _station_bump) =
        Pubkey::find_program_address(&[weather_station::WEATHER_SEED], &WEATHER_STATION_PID);

    // TODO: fetch or initialize the station account

    let mut scheduler = AsyncScheduler::with_tz(chrono::Utc);

    // // heartbeat every 60 seconds
    // scheduler.every(60.seconds()).run(|| async move {});

    // update the weather reports every 5 min
    scheduler.every(5.minutes()).run(move || {
        let my_cluster = cluster.clone();
        let my_payer = payer.clone();
        let my_quotekp = Arc::clone(&quotekp);

        run_save_report(station, my_cluster, my_payer, my_quotekp)
    });

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(10));
    }
}

async fn run_save_report(
    station: Pubkey,
    cluster: anchor_client::Cluster,
    payer: Arc<Keypair>,
    quotekp: Arc<Keypair>,
) {
    // fetch reports
    let report_result = fetch_weather_report().await;
    if report_result.is_err() {
        // log and exit
        log::warn!("Failed to fetch weather reports");
        return;
    }

    // save reports
    let save_result = save_report(
        &anchor_client::Client::new_with_options(cluster, payer, CommitmentConfig::processed()),
        quotekp,
        station,
        report_result.unwrap(),
    );
    if save_result.is_err() {
        // log and exit
        log::warn!("Failed to save weather reports");
    }
}

fn save_report<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    quotekp: Arc<Keypair>,
    station: Pubkey,
    reports: [weather_station::WeatherReport; 4],
) -> Result<(), ClientError> {
    let program = client.program(WEATHER_STATION_PID);
    program
        .request()
        .signer(quotekp.deref())
        .accounts(weather_station::accounts::Report { station })
        .args(weather_station::instruction::Report {
            params: weather_station::ReportParams { reports },
        })
        .send()?;

    Ok(())
}
