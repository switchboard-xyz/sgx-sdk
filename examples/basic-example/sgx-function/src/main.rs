use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_sdk::signature::Signer;
use anchor_client::solana_sdk::signer::keypair::keypair_from_seed;
use anchor_client::solana_sdk::signer::keypair::Keypair;
use anchor_client::Client;
use anchor_client::Cluster;
use sbac::sgx::Sgx;
use sbac::solana::*;

use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::read_keypair_file;
use std::sync::Arc;
use switchboard_attestation_client as sbac;
use tokio;

const VERIFIER_QUEUE: Pubkey = pubkey!("4AnQSCo6YJmmYVq2BUHx5vfxoqktpBmTbDh1NurxShgN");
const PID: Pubkey = pubkey!("Hxfwq7cxss4Ef9iDvaLb617dhageGyNWbDLLrg2sdQgT");
type AnchorClient = Client<Arc<Keypair>>;

pub async fn solana_init_quote(anchor_client: &AnchorClient, payer: Arc<Keypair>) -> Arc<Keypair> {
    let mut randomness = [0; 32];
    Sgx::read_rand(&mut randomness).unwrap();
    let quote_kp = Arc::new(keypair_from_seed(&randomness).unwrap());
    let quote = Sgx::gramine_generate_quote(&quote_kp.pubkey().to_bytes()).unwrap();
    let quote_init_ixs = QuoteInitSimple::build(
        &anchor_client,
        QuoteInitSimpleArgs {
            quote: quote_kp.pubkey(),
            verifier_queue: VERIFIER_QUEUE,
            authority: quote_kp.pubkey(),
            data: quote.clone(),
        },
        vec![&payer, &quote_kp],
    )
    .unwrap();
    let rpc = anchor_client.program(PID).rpc();
    let blockhash = rpc.get_latest_blockhash().unwrap();
    let mut sigs = Vec::new();
    for (i, ix) in quote_init_ixs.iter().enumerate() {
        println!("Trying quote init {}", i);
        let tx = ix_to_tx(&[ix.clone()], &[&payer, &quote_kp], blockhash);
        let sig = rpc.send_transaction(&tx).unwrap();
        println!("Quote init {}", sig);
        sigs.push(sig);
    }
    for sig in sigs {
        rpc.poll_for_signature_confirmation(&sig, 20).unwrap();
        println!("{} confirmed", sig);
    }
    quote_kp
}

#[tokio::main(worker_threads = 12)]
async fn main() {
    let url = std::env::var("RPC_URL").unwrap();
    let wss_url = url.replace("https://", "wss://");
    let cluster = Cluster::Custom(url.clone(), wss_url.clone());
    let payer = Arc::new(read_keypair_file(std::env::var("PAYER").unwrap()).unwrap());
    let anchor_client = anchor_client::Client::new_with_options(
        cluster,
        payer.clone(),
        CommitmentConfig::processed(),
    );
    let quotekp = solana_init_quote(&anchor_client, payer.clone()).await;
}
