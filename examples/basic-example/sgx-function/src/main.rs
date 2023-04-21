use anchor_client::solana_sdk::signer::keypair::Keypair;
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

type AnchorClient = anchor_client::Client<Arc<Keypair>>;

#[tokio::main(worker_threads = 12)]
async fn main() {
    // setup logging
    simple_logger::init_with_env().unwrap();

    let url = std::env::var("RPC_URL").unwrap();
    let wss_url = url.replace("https://", "wss://");
    let cluster = anchor_client::Cluster::Custom(url.clone(), wss_url.clone());
    let payer = Arc::new(read_keypair_file(std::env::var("PAYER").unwrap()).unwrap());
    let anchor_client = anchor_client::Client::new_with_options(
        cluster.clone(),
        payer.clone(),
        CommitmentConfig::processed(),
    );
    let client = Arc::new(anchor_client);

    let quotekp = switchboard::run_init_quote(client.clone(), payer.clone()).await;

    let (station, _station_bump) =
        Pubkey::find_program_address(&[weather_station::WEATHER_SEED], &weather_station::id());

    // TODO: Verify the station account has been created

    let mut scheduler = AsyncScheduler::with_tz(chrono::Utc);

    // update the weather reports every 5 min
    scheduler.every(5.minutes()).run(move || {
        let my_cluster = cluster.clone();
        let my_payer = payer.clone();
        let my_quotekp = Arc::clone(&quotekp);

        run_save_report(station, my_cluster, my_payer, my_quotekp)
    });

    // TODO: Add heartbeat task
    scheduler
        .every(60.seconds())
        .run(move || switchboard::run_heartbeat(client.clone()));

    // TODO: Add quote publish task every 24 hrs ?? (Might not be needed)

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(10));
    }
}

pub async fn run_save_report(
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
    let client =
        anchor_client::Client::new_with_options(cluster, payer, CommitmentConfig::processed());
    let program = client.program(weather_station::id());
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
