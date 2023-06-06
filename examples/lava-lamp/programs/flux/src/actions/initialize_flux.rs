use crate::*;

#[derive(Accounts)]
#[instruction(params: InitializeFluxParams)] // rpc parameters hint
pub struct InitializeFlux<'info> {
    #[account(
        init,
        space = 8 + std::mem::size_of::<FluxAccount>(),
        payer = payer,
        seeds = [b"FLUXSTATE"], 
        bump
    )]
    pub flux_state: AccountLoader<'info, FluxAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,

    /// CHECK:
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: AccountInfo<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeFluxParams {}

impl InitializeFlux<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, params: &InitializeFluxParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: &Context<Self>, params: &InitializeFluxParams) -> Result<()> {
        let bump = *ctx.bumps.get("flux_state").unwrap();

        let flux = &mut ctx.accounts.flux_state.load_init()?;
        flux.bump = bump;

        Ok(())
    }
}
