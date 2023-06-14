pub mod binance;
use binance::{fetch_prices, Ticker};

pub use switchboard_solana::prelude::*;

use binance_oracle::{self, PushDataParams};

use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main(worker_threads = 12)]
async fn main() {
    let fn_signer = generate_signer();
    let fn_signer_pubkey = Pubkey::from_str(fn_signer.to_base58_string().as_str())
        .expect("failed to convert base58 pubkey");

    let fn_accounts =
        FunctionVerifyPubkeys::load_from_env().expect("failed to load function env variables");

    let (state_pubkey, _bump) =
        Pubkey::find_program_address(&[b"BINANCEORACLE"], &binance_oracle::ID);

    let symbols = ["BTCUSDC", "ETHUSDC", "SOLUSDC"];
    let tickers: Vec<Ticker> = fetch_prices(symbols.to_vec()).await.unwrap();
    println!("{:#?}", tickers);

    let ixns: Vec<solana_program::instruction::Instruction> = symbols
        .iter()
        .enumerate()
        .map(|(i, s)| {
            build_ix(
                s,
                state_pubkey,
                tickers.get(i).unwrap(),
                &fn_accounts,
                fn_signer_pubkey,
            )
        })
        .collect();

    let result = function_verify("https://api.devnet.solana.com".to_string(), fn_signer, ixns)
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

fn build_ix(
    symbol_str: &str,
    state_pubkey: Pubkey,
    ticker: &Ticker,
    fn_accounts: &FunctionVerifyPubkeys,
    fn_signer_pubkey: Pubkey,
) -> solana_program::instruction::Instruction {
    let symbol = string_to_bytes(symbol_str);

    let (oracle_pubkey, _oracle_bump) =
        Pubkey::find_program_address(&[state_pubkey.as_ref(), &symbol[..]], &binance_oracle::ID);

    let price = ticker.lastPrice.parse::<f64>().unwrap();
    let volume = ticker.volume.parse::<f64>().unwrap();

    solana_program::instruction::Instruction {
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
                pubkey: fn_accounts.function,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: fn_accounts.quote,
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
                pubkey: solana_program::system_program::ID,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: solana_program::sysvar::rent::ID,
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
    }
}
