use std::str::FromStr;

use anchor_lang::prelude::*;
use solana_sdk::{instruction::Instruction, pubkey, pubkey::Pubkey};
use switchboard_solana::{function_verify, generate_signer};

const PROGRAM_ID: Pubkey = pubkey!("5dx7h7Sm7JWki5SWqHMVUMKykheNWLL8JKx3rq4ubYA9");

// TODO: update
const LAVA_LAMP_STATE_PUBKEY: Pubkey = pubkey!("5dx7h7Sm7JWki5SWqHMVUMKykheNWLL8JKx3rq4ubYA9");
const BUFFER_PUBKEY: Pubkey = pubkey!("5dx7h7Sm7JWki5SWqHMVUMKykheNWLL8JKx3rq4ubYA9");
const FUNCTION_PUBKEY: Pubkey = pubkey!("5dx7h7Sm7JWki5SWqHMVUMKykheNWLL8JKx3rq4ubYA9");
const QUOTE_PUBKEY: Pubkey = pubkey!("5dx7h7Sm7JWki5SWqHMVUMKykheNWLL8JKx3rq4ubYA9");

#[tokio::main(worker_threads = 12)]
async fn main() {
    let fn_signer = generate_signer();
    let fn_signer_pubkey = Pubkey::from_str(fn_signer.to_base58_string().as_str())
        .expect("failed to convert base58 pubkey");

    let mut bytes: [u8; 500] = [0u8; 500];
    switchboard_common::Gramine::read_rand(&mut bytes)
        .expect("gramine failed to generate randomness");

    let ixn = Instruction {
        program_id: PROGRAM_ID,
        data: bytes.to_vec(),
        accounts: vec![
            AccountMeta {
                pubkey: LAVA_LAMP_STATE_PUBKEY,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: BUFFER_PUBKEY,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: FUNCTION_PUBKEY,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: QUOTE_PUBKEY,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: fn_signer_pubkey,
                is_signer: true,
                is_writable: false,
            },
        ],
    };

    let ixns = vec![ixn];

    let result = function_verify(fn_signer, ixns)
        .await
        .expect("failed to run function verify");

    result.emit();
}
