use crate::{AnchorClient, AnchorProgram, Keypair};
use std::sync::Arc;

use anchor_client::solana_sdk::signature::Signer;
use anchor_client::solana_sdk::signer::keypair::keypair_from_seed;
use solana_sdk::{pubkey, pubkey::Pubkey};

use switchboard_common::Gramine;

pub const PID: Pubkey = pubkey!("SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f");

const VERIFIER_QUEUE: Pubkey = pubkey!("4AnQSCo6YJmmYVq2BUHx5vfxoqktpBmTbDh1NurxShgN");

pub async fn run_init_quote(client: AnchorClient, payer: Keypair) -> Keypair {
    let mut randomness = [0; 32];
    Gramine::read_rand(&mut randomness).unwrap();
    let quote_kp = Arc::new(keypair_from_seed(&randomness).unwrap());
    let quote = Gramine::generate_quote(&quote_kp.pubkey().to_bytes()).unwrap();
    // let quote_init_ixs = QuoteInitSimple::build(
    //     &client,
    //     QuoteInitSimpleArgs {
    //         quote: quote_kp.pubkey(),
    //         verifier_queue: VERIFIER_QUEUE,
    //         authority: quote_kp.pubkey(),
    //         data: quote,
    //     },
    //     vec![&payer, &quote_kp],
    // )
    // .unwrap();
    // let rpc = client.program(PID).rpc();
    // let blockhash = rpc.get_latest_blockhash().unwrap();
    // let mut sigs = Vec::new();
    // for (i, ix) in quote_init_ixs.iter().enumerate() {
    //     println!("Trying quote init {}", i);
    //     let tx = ix_to_tx(&[ix.clone()], &[&payer, &quote_kp], blockhash);
    //     let sig = rpc.send_transaction(&tx).unwrap();
    //     println!("Quote init {}", sig);
    //     sigs.push(sig);
    // }
    // for sig in sigs {
    //     rpc.poll_for_signature_confirmation(&sig, 20).unwrap();
    //     println!("{} confirmed", sig);
    // }
    quote_kp
}

pub async fn run_heartbeat(program: AnchorProgram) {}
