// Note: Binance API requires a non-US IP address

use anchor_lang::prelude::*;
use anchor_lang::AnchorSerialize;
use binance_oracle::{self, PushDataParams};
use serde::Deserialize;
use solana_sdk::{instruction::Instruction, pubkey, pubkey::Pubkey};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use switchboard_solana::{function_verify, generate_signer, FunctionVerifyPubkeys};

const FUNCTION_PUBKEY: Pubkey = pubkey!("5dx7h7Sm7JWki5SWqHMVUMKykheNWLL8JKx3rq4ubYA9");
const QUOTE_PUBKEY: Pubkey = pubkey!("5dx7h7Sm7JWki5SWqHMVUMKykheNWLL8JKx3rq4ubYA9");

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
struct Ticker {
    pub symbol: String,
    pub weightedAvgPrice: String,
    pub lastPrice: String,
    pub volume: String,
}

#[tokio::main(worker_threads = 12)]
async fn main() {
    let fn_accounts =
        FunctionVerifyPubkeys::load_from_env().expect("failed to load function env variables");

    let fn_signer = generate_signer();
    let fn_signer_pubkey = Pubkey::from_str(fn_signer.to_base58_string().as_str())
        .expect("failed to convert base58 pubkey");

    let (state_pubkey, _bump) =
        Pubkey::find_program_address(&[b"BINANCEORACLE"], &binance_oracle::ID);

    let symbols = ["BTCUSDC"];
    let symbol = string_to_bytes("BTCUSDC");
    let (oracle_pubkey, oracle_bump) =
        Pubkey::find_program_address(&[state_pubkey.as_ref(), &symbol[..]], &binance_oracle::ID);

    let symbols = symbols.map(|x| format!("\"{}\"", x)).join(",");
    let tickers = reqwest::get(format!(
        "https://api.binance.com/api/v3/ticker?symbols=[{}]&windowSize=1h",
        symbols
    ))
    .await
    .unwrap()
    .json::<Vec<Ticker>>()
    .await
    .unwrap();

    let btc_ticker = tickers.get(0).unwrap();

    let price = btc_ticker.lastPrice.parse::<f64>().unwrap();
    let volume = btc_ticker.volume.parse::<f64>().unwrap();

    println!("{:#?}", tickers);

    let ixn = Instruction {
        program_id: binance_oracle::ID,
        accounts: vec![
            AccountMeta {
                pubkey: state_pubkey,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: oracle_pubkey,
                is_signer: false,
                is_writable: false,
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
            AccountMeta {
                pubkey: fn_accounts.payer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: anchor_lang::solana_program::system_program::ID,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: anchor_lang::solana_program::sysvar::rent::ID,
                is_signer: false,
                is_writable: false,
            },
        ],
        data: PushDataParams {
            symbol,
            price,
            volume,
            oracle_timestamp: unix_timestamp(),
        }
        .try_to_vec()
        .unwrap_or_default(),
    };

    let ixns = vec![ixn];

    let result = function_verify(fn_signer, ixns)
        .await
        .expect("failed to run function verify");

    result.emit();
}

fn string_to_bytes(s: &str) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = s.as_bytes();

    for (&x, p) in bytes.iter().zip(array.iter_mut()) {
        *p = x;
    }

    if bytes.len() > 32 {
        eprintln!("Warning: string was longer than 32 bytes, it has been truncated");
    }

    array
}

fn unix_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .try_into()
        .unwrap_or(0)
}
