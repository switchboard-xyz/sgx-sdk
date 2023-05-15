use crate::error::Err;
use anchor_client::anchor_lang;
use anchor_client::anchor_lang::prelude::*;
use anchor_client::anchor_lang::Discriminator;
use anchor_client::anchor_lang::InstructionData;
pub use anchor_client::solana_sdk;
use anchor_client::solana_sdk::hash::Hash;
use anchor_client::solana_sdk::instruction::Instruction;
use anchor_client::solana_sdk::message::Message;
use anchor_client::solana_sdk::pubkey;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::Signer;
use anchor_client::solana_sdk::signer::keypair::Keypair;
use anchor_client::solana_sdk::transaction::Transaction;
use bytemuck::Pod;
use bytemuck::Zeroable;
use std::result::Result;
use std::sync::Arc;

pub struct QuoteInitSimple {
    pub quote: Pubkey,
    pub verifier_queue: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct QuoteInitSimpleParams {
    pub data: [u8; 512],
    pub total_len: u32,
    pub chunk_start: u32,
    pub chunk_end: u32,
}
pub struct QuoteInitSimpleArgs {
    pub quote: Pubkey,
    pub verifier_queue: Pubkey,
    pub authority: Pubkey,
    pub data: Vec<u8>,
}
impl Discriminator for QuoteInitSimpleParams {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
    fn discriminator() -> [u8; 8] {
        ix_discriminator("quote_init_simple")
    }
}
impl InstructionData for QuoteInitSimpleParams {}
impl ToAccountMetas for QuoteInitSimple {
    fn to_account_metas(&self, _: Option<bool>) -> Vec<AccountMeta> {
        vec![
            AccountMeta {
                pubkey: self.quote,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: self.verifier_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: self.authority,
                is_signer: false, // TODO: Make conditional if in signers
                is_writable: false,
            },
            AccountMeta {
                pubkey: self.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: self.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl QuoteInitSimple {
    pub fn build(
        client: &anchor_client::Client<Arc<Keypair>>,
        args: QuoteInitSimpleArgs,
        signers: Vec<&Keypair>,
    ) -> Result<Vec<Instruction>, Err> {
        let mut ixs = Vec::new();
        let payer = signers[0];
        let mut i = 0;
        let data = args.data;
        while i < data.len() {
            let back = std::cmp::min(i + 512, data.len());
            let mut chunk = data[i..back].to_vec();
            chunk.resize(512, 0);
            ixs.push(build_ix(
                QuoteInitSimple {
                    quote: args.quote,
                    verifier_queue: args.verifier_queue,
                    authority: args.authority,
                    payer: payer.pubkey(),
                    system_program: anchor_client::solana_sdk::system_program::ID,
                },
                QuoteInitSimpleParams {
                    data: chunk.try_into().unwrap(),
                    total_len: data.len() as u32,
                    chunk_start: i as u32,
                    chunk_end: back as u32,
                },
            ));
            i += 512;
        }
        Ok(ixs)
    }
}

pub const PID: Pubkey = pubkey!("Hxfwq7cxss4Ef9iDvaLb617dhageGyNWbDLLrg2sdQgT");

pub fn ix_discriminator(name: &str) -> [u8; 8] {
    let preimage = format!("global:{}", name);
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8],
    );
    sighash
}

pub fn ix_to_tx(ixs: &[Instruction], signers: &[&Keypair], blockhash: Hash) -> Transaction {
    let msg = Message::new(ixs, Some(&signers[0].pubkey()));
    Transaction::new(&signers.to_vec(), msg, blockhash)
}

pub fn build_ix<A: ToAccountMetas, I: InstructionData + Discriminator>(
    accounts: A,
    params: I,
) -> Instruction {
    Instruction {
        program_id: PID,
        accounts: accounts.to_account_metas(None),
        data: params.data(),
    }
}

pub fn build_tx<A: ToAccountMetas, I: InstructionData + Discriminator>(
    anchor_client: &anchor_client::Client<Arc<Keypair>>,
    accounts: A,
    params: I,
    signers: Vec<&Keypair>,
) -> Transaction {
    let payer = signers[0];
    let ix = Instruction {
        program_id: PID,
        accounts: accounts.to_account_metas(None),
        data: params.data(),
    };
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    let blockhash = anchor_client
        .program(PID)
        .rpc()
        .get_latest_blockhash()
        .unwrap();
    tx.sign(&signers, blockhash);
    println!("{:?}", tx.message.account_keys);
    tx
}

pub fn load<T: bytemuck::Pod>(
    client: &anchor_client::Client<Arc<Keypair>>,
    key: Pubkey,
) -> Result<T, Err> {
    let data = client.program(PID).rpc().get_account_data(&key).unwrap();
    Ok(*bytemuck::from_bytes::<T>(&data[8..]))
}

// TODO: Verify zero_copy works
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct ServiceQueueAccountData {
    // Authority controls adding/removing allowed enclave measurements
    pub authority: Pubkey,
    // All attestation queues will have verifier_queue == self
    pub verifier_queue: Pubkey,
    // allowed enclave measurements
    pub mr_enclaves: [[u8; 32]; 32],
    pub mr_enclaves_len: u32,
    pub data: [Pubkey; 32],
    pub data_len: u32,
    // Allow authority to force add a node after X seconds with no heartbeat
    pub allow_authority_override_after: i64,
    // Even if a heartbeating machine quote verifies with proper measurement,
    // require authority signoff.
    pub require_authority_heartbeat_permission: bool,
    pub require_usage_permissions: bool,
    pub max_quote_verification_age: i64,
    pub reward: u32, //TODO
    pub last_heartbeat: i64,
    pub node_timeout: i64,
    pub curr_idx: u32,
    pub gc_idx: u32,
    pub _ebuf: [u8; 1024],
}
unsafe impl Pod for ServiceQueueAccountData {}
unsafe impl Zeroable for ServiceQueueAccountData {}
