use anchor_client::solana_sdk::signer::keypair::Keypair;

use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signer::keypair::read_keypair_file;
use solana_sdk::{pubkey, pubkey::Pubkey};
use std::sync::Arc;
use tokio;

pub mod fetch;
pub mod init;

pub use fetch::*;
pub use init::*;

const VERIFIER_QUEUE: Pubkey = pubkey!("4AnQSCo6YJmmYVq2BUHx5vfxoqktpBmTbDh1NurxShgN");
const PID: Pubkey = pubkey!("Hxfwq7cxss4Ef9iDvaLb617dhageGyNWbDLLrg2sdQgT");

type AnchorClient = anchor_client::Client<Arc<Keypair>>;

#[tokio::main(worker_threads = 12)]
async fn main() {
    let url = std::env::var("RPC_URL").unwrap();
    let wss_url = url.replace("https://", "wss://");
    let cluster = anchor_client::Cluster::Custom(url.clone(), wss_url.clone());
    let payer = Arc::new(read_keypair_file(std::env::var("PAYER").unwrap()).unwrap());
    let anchor_client = anchor_client::Client::new_with_options(
        cluster,
        payer.clone(),
        CommitmentConfig::processed(),
    );
    let quotekp = solana_init_quote(&anchor_client, payer.clone()).await;

    // setup scheduling
}
