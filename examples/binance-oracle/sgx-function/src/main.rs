use std::str::FromStr;

use anchor_lang::prelude::*;
use solana_sdk::{instruction::Instruction, pubkey, pubkey::Pubkey};
use switchboard_solana::{function_verify, generate_signer};

const DEMO_PID: Pubkey = pubkey!("4ih7pcGkVT2HBJuXqTFemhyd73BQktBkuKXyrbRZn22v");

// #[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, Default)]
// pub struct PingParams {
//     pub prices: Vec<BorshDecimal>,
//     pub volumes: Vec<BorshDecimal>,
//     pub twaps: Vec<BorshDecimal>,
// }

#[tokio::main(worker_threads = 12)]
async fn main() {
    let fn_signer = generate_signer();
    let fn_signer_pubkey = Pubkey::from_str(fn_signer.to_base58_string().as_str())
        .expect("failed to convert base58 pubkey");

    // let ixn = Instruction {
    //     program_id: DEMO_PID,
    //     accounts: vec![
    //         AccountMeta {
    //             pubkey: fn_key,
    //             is_signer: false,
    //             is_writable: false,
    //         },
    //         AccountMeta {
    //             pubkey: fn_quote,
    //             is_signer: false,
    //             is_writable: false,
    //         },
    //         AccountMeta {
    //             pubkey: enclave_signer.pubkey(),
    //             is_signer: true,
    //             is_writable: false,
    //         },
    //     ],
    //     data: PingParams {
    //         prices: tickers
    //             .iter()
    //             .map(|x| BorshDecimal::from(&x.lastPrice))
    //             .collect(),
    //         volumes: tickers
    //             .iter()
    //             .map(|x| BorshDecimal::from(&x.volume))
    //             .collect(),
    //         twaps: tickers
    //             .iter()
    //             .map(|x| BorshDecimal::from(&x.weightedAvgPrice))
    //             .collect(),
    //     }
    //     .data(),
    // };

    // let ixns = vec![ixn];

    // let result = function_verify(fn_signer, ixns)
    //     .await
    //     .expect("failed to run function verify");

    // result.emit();
}
