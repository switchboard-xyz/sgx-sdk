use crate::{AnchorClient, Error, Keypair};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::read_keypair_file;
use std::str::FromStr;
use std::sync::Arc;

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default)]
pub struct Env {
    pub RPC_URL: String,
    pub WS_URL: String,
    pub PROGRAM_ID: String,
    pub QUEUE: String,
    pub PAYER_SECRET_PATH: String,
}

impl std::fmt::Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "=====Node Environment=====")?;
        write!(f, "RPC_URL:                  {}", self.RPC_URL)?;
        write!(f, "WS_URL:                  {}", self.WS_URL)?;
        write!(f, "PROGRAM_ID:                    {}", self.PROGRAM_ID)?;
        write!(f, "QUEUE:                    {}", self.QUEUE)?;
        write!(f, "PAYER_SECRET_PATH: {}", self.PAYER_SECRET_PATH)?;
        write!(f, "=========================")?;
        Ok(())
    }
}

impl Env {
    pub fn new() -> Result<Self, Error> {
        let rpc_url = std::env::var("RPC_URL").unwrap_or(String::new());
        if rpc_url.is_empty() {
            return Err(Error::ConfigError("RPC_URL cannot be empty".to_string()));
        }

        let ws_url = rpc_url.replace("https://", "wss://");

        let program_id = std::env::var("PROGRAM_ID").unwrap_or(String::new());
        if program_id.is_empty() {
            return Err(Error::ConfigError("PROGRAM_ID cannot be empty".to_string()));
        }

        let queue = std::env::var("QUEUE").unwrap_or(String::new());
        if queue.is_empty() {
            return Err(Error::ConfigError("QUEUE cannot be empty".to_string()));
        }

        let payer_secret_path = std::env::var("FS_PAYER_SECRET_PATH").unwrap_or(String::new());
        if payer_secret_path.is_empty() {
            return Err(Error::ConfigError(
                "FS_PAYER_SECRET_PATH cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            RPC_URL: rpc_url,
            WS_URL: ws_url,
            PROGRAM_ID: program_id,
            QUEUE: queue,
            PAYER_SECRET_PATH: payer_secret_path,
        })
    }
}

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct Config {
    pub env: Env,
    pub program_id: Pubkey,
    pub queue: Pubkey,
    pub client: AnchorClient,
    pub payer: Keypair,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        let env = Env::new()?;

        let program_id_key = Pubkey::from_str(env.PROGRAM_ID.as_str())
            .map_err(|e| Error::CustomMessage(e.to_string()))?;

        let queue_key = Pubkey::from_str(env.QUEUE.as_str())
            .map_err(|e| Error::CustomMessage(e.to_string()))?;

        let payer: Keypair = Arc::new(read_keypair_file(env.PAYER_SECRET_PATH.clone()).unwrap());

        let cluster = anchor_client::Cluster::Custom(env.RPC_URL.clone(), env.WS_URL.clone());
        let client: AnchorClient = Arc::new(anchor_client::Client::new_with_options(
            cluster,
            payer.clone(),
            CommitmentConfig::processed(),
        ));

        Ok(Self {
            env,
            program_id: program_id_key,
            queue: queue_key,
            payer,
            client,
        })
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "=====Node Environment=====")?;
        write!(f, "RPC_URL:                  {}", self.env.RPC_URL)?;
        write!(f, "WS_URL:                  {}", self.env.WS_URL)?;
        write!(f, "PROGRAM_ID:                    {}", self.env.PROGRAM_ID)?;
        write!(f, "QUEUE:                    {}", self.env.QUEUE)?;
        write!(f, "PAYER_SECRET_PATH: {}", self.env.PAYER_SECRET_PATH)?;
        write!(f, "=========================")?;
        Ok(())
    }
}
