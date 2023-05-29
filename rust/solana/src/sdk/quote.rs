use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};

#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct QuoteAccountData {
    // If this key is not Pubkey::default, then this is the secured
    // signing key rather than the account key itself
    // Set for functions only
    /// TODO: Add description
    pub delegated_secured_signer: Pubkey,
    pub bump: u8,
    // Set except for function quotes
    /// TODO: Add description
    pub quote_registry: [u8; 32],
    /// Key to lookup the buffer data on IPFS or an alternative decentralized storage solution.
    pub registry_key: [u8; 64],

    // always set
    /// Queue used for attestation to verify a MRENCLAVE measurement.
    pub attestation_queue: Pubkey,
    /// The quotes MRENCLAVE measurement dictating the contents of the secure enclave.
    pub mr_enclave: [u8; 32],
    pub verification_status: u8,
    pub verification_timestamp: i64,
    pub valid_until: i64,
    // Set for verifiers
    pub is_on_queue: bool,
    /// The last time the quote heartbeated.
    pub last_heartbeat: i64,
    pub owner: Pubkey,
    //
    pub created_at: i64,
    pub _ebuf: [u8; 992],
}

impl QuoteAccountData {}
