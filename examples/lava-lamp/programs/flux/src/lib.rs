pub use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::Discriminator;

pub mod actions;
pub use actions::*;

declare_id!("GxvdVxsf8vftqEzdQBHefq1s42PpoM8SeCtG2yay1oCm");

pub const STATE_SEED: &[u8] = b"FLUXSTATE";

#[program]
pub mod flux {
    use super::*;

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn initialize_flux(
        ctx: Context<InitializeFlux>,
        params: InitializeFluxParams,
    ) -> Result<()> {
        InitializeFlux::actuate(&ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn read(ctx: Context<ReadFlux>, params: ReadFluxParams) -> Result<()> {
        ReadFlux::actuate(&ctx, &params)
    }
}

#[repr(packed)]
#[account(zero_copy)]
pub struct FluxAccount {
    /// The PDA bump.
    pub bump: u8,
    /// The random value we store on this account.
    pub value: u64,
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum FluxError {
    #[msg("Something bad happened.")]
    GenericError,
}
