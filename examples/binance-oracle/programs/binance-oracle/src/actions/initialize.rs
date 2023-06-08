use crate::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        space = 8 + std::mem::size_of::<ProgramState>(),
        payer = payer,
        seeds = [PROGRAM_SEED],
        bump
    )]
    pub state: AccountLoader<'info, ProgramState>,

    /// CHECK:
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,

    pub attestation_queue: AccountLoader<'info, AttestationQueueAccountData>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,

    /// CHECK:
    #[account(address = anchor_lang::solana_program::sysvar::rent::ID)]
    pub rent: AccountInfo<'info>,
}

impl Initialize<'_> {
    pub fn actuate(ctx: &Context<Self>) -> Result<()> {
        let state = &mut ctx.accounts.state.load_init()?;

        let bump = ctx.bumps.get("state").unwrap();
        state.bump = *bump;

        // TODO: validate queue params
        state.attestation_queue = ctx.accounts.attestation_queue.key();

        Ok(())
    }
}
