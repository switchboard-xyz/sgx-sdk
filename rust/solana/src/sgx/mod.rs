pub use anchor_client;

use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::signer::keypair::{keypair_from_seed, Keypair};
use anchor_client::solana_sdk::transaction::Transaction;
use anchor_client::Cluster;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::message::Message;
use anchor_lang::solana_program::pubkey::Pubkey;
use sgx_quote::Quote;
use std::env;
use std::result::Result;
use std::str::FromStr;
use std::sync::Arc;
use switchboard_common::{Error, FunctionResult, Gramine};

// use crate::{FunctionVerify, SWITCHBOARD_ATTESTATION_PROGRAM_ID};

use crate::attestation_program::FunctionVerify;
use crate::SWITCHBOARD_ATTESTATION_PROGRAM_ID;

pub fn generate_signer() -> Arc<Keypair> {
    let mut randomness = [0; 32];
    Gramine::read_rand(&mut randomness).unwrap();
    Arc::new(keypair_from_seed(&randomness).unwrap())
}

pub async fn function_verify(
    enclave_signer: Arc<Keypair>,
    mut ixs: Vec<Instruction>,
) -> Result<FunctionResult, Error> {
    let client = anchor_client::Client::new_with_options(
        Cluster::Devnet,
        enclave_signer.to_owned(),
        CommitmentConfig::processed(),
    );
    let enclave_signer_pubkey = to_pubkey(enclave_signer.clone())?;
    let quote_raw = Gramine::generate_quote(&enclave_signer_pubkey.to_bytes()).unwrap();
    let quote = Quote::parse(&quote_raw).unwrap();

    let blockhash = client
        .program(SWITCHBOARD_ATTESTATION_PROGRAM_ID)
        .rpc()
        .get_latest_blockhash()
        .unwrap();

    let pubkeys = FunctionVerifyPubkeys::load_from_env()?;

    let ix = FunctionVerify::build(
        &client,
        enclave_signer.clone(),
        &pubkeys,
        quote.isv_report.mrenclave.try_into().unwrap(),
    )
    .await?;

    ixs.insert(0, ix);

    let message = Message::new(&ixs, Some(&pubkeys.payer));

    let mut tx = Transaction::new_unsigned(message);
    tx.partial_sign(&[enclave_signer.as_ref()], blockhash);

    Ok(FunctionResult {
        version: 1,
        chain: switchboard_common::Chain::Solana,
        key: pubkeys.function.to_bytes(),
        signer: enclave_signer_pubkey.to_bytes(),
        serialized_tx: bincode::serialize(&tx).unwrap(),
        quote: quote_raw,
        ..Default::default()
    })
}

pub async fn load_account<T: bytemuck::Pod>(
    client: &anchor_client::Client<Arc<Keypair>>,
    pubkey: Pubkey,
) -> Result<T, switchboard_common::Error> {
    let data = client
        .program(SWITCHBOARD_ATTESTATION_PROGRAM_ID)
        .async_rpc()
        .get_account_data(&pubkey)
        .await
        .map_err(|_| switchboard_common::Error::CustomMessage("AnchorParseError".to_string()))?;
    Ok(*bytemuck::try_from_bytes::<T>(&data[8..])
        .map_err(|_| switchboard_common::Error::CustomMessage("AnchorParseError".to_string()))?)
}

pub struct FunctionVerifyPubkeys {
    pub function: Pubkey,
    pub payer: Pubkey,
    pub verifier: Pubkey,
    pub reward_receiver: Pubkey,
}
impl FunctionVerifyPubkeys {
    pub fn load_from_env() -> std::result::Result<Self, switchboard_common::Error> {
        let function = Pubkey::from_str(&env::var("FUNCTION_KEY").unwrap()).unwrap();
        let payer = Pubkey::from_str(&env::var("PAYER").unwrap()).unwrap();

        let verifier = &env::var("VERIFIER").unwrap_or(String::new());
        if verifier.is_empty() {
            return Err(switchboard_common::Error::CustomMessage(
                "verifier missing".to_string(),
            ));
        }

        Ok(Self {
            function,
            payer,
            verifier: Pubkey::from_str(verifier).map_err(|_| {
                switchboard_common::Error::CustomMessage(
                    "failed to parse pubkey string".to_string(),
                )
            })?,
            reward_receiver: Pubkey::from_str(&env::var("REWARD_RECEIVER").unwrap()).unwrap(),
        })
    }
}

pub fn to_pubkey(signer: Arc<Keypair>) -> std::result::Result<Pubkey, switchboard_common::Error> {
    let pubkey = Pubkey::from_str(signer.to_base58_string().as_str()).map_err(|_| {
        switchboard_common::Error::CustomMessage("failed to parse pubkey string".to_string())
    })?;
    Ok(pubkey)
}
