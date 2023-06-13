use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use switchboard_solana::FunctionAccountData;
use switchboard_solana::QuoteAccountData;

pub mod actions;
pub use actions::*;

declare_id!("DpLmXhwhbQP3NhUKpnWgbteUGpoWAETsJEzGntiKHrN1");

pub const STATE_SEED: &[u8] = b"LAVALAMPSTATE";

#[program]
pub mod lava_lamp {
    use super::*;

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        Initialize::actuate(&ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn bubble(ctx: Context<Bubble>, params: BubbleParams) -> Result<()> {
        Bubble::actuate(&ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn read_randomness(
        ctx: Context<ReadRandomness>,
        params: ReadRandomnessParams,
    ) -> Result<()> {
        ReadRandomness::actuate(&ctx, &params)
    }
}

#[repr(packed)]
#[account(zero_copy(unsafe))]
pub struct LavaLampAccount {
    /// The PDA bump.
    pub bump: u8,
    /// The account schema version.
    pub schema: u8,
    /// The unix timestamp when the randomness buffer was last updated.
    pub last_updated: i64,
    /// The cost to consume a byte of randomness.
    pub byte_cost: u64,
    /// The size of the buffer.
    pub max_size: u32,
    /// The idx of the last consumed randomness value.
    pub read_idx: u32,
    /// The authority authorized to make account changes.
    pub authority: Pubkey,
    /// The pubkey of the buffer account.
    pub buffer: Pubkey,
    /// The pubkey of the token account to receive rewards.
    pub reward_address: Pubkey,
}

#[repr(packed)]
#[account(zero_copy(unsafe))]
pub struct BufferAccount {
    /// The idx of the last element written to the buffer.
    pub write_idx: u32,
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum Meltdown {
    #[msg("Trusted signer does not match quote account")]
    InvalidTrustedSigner,
}
