use crate::*;

#[derive(Accounts)]
#[instruction(params: InitializeParams)] // rpc parameters hint
pub struct Initialize<'info> {
    #[account(
        init,
        space = 8 + std::mem::size_of::<LavaLampAccount>(),
        payer = payer,
        seeds = [b"LAVALAMPSTATE"], 
        bump
    )]
    pub lava_lamp: AccountLoader<'info, LavaLampAccount>,

    #[account(
        init,
        space = 8 + std::mem::size_of::<BufferAccount>() + params.buffer_size as usize,
        payer = payer
    )]
    pub buffer: AccountLoader<'info, BufferAccount>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = lava_lamp,
    )]
    pub reward_address: Box<Account<'info, TokenAccount>>,

    #[account(address = anchor_spl::token::spl_token::native_mint::ID)]
    pub mint: Account<'info, Mint>,

    /// CHECK:
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK:
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: AccountInfo<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeParams {
    pub buffer_size: u32,
    pub byte_cost: u64,
}

impl Initialize<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, params: &InitializeParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: &Context<Self>, params: &InitializeParams) -> Result<()> {
        let bump = *ctx.bumps.get("lava_lamp").unwrap();

        let lava_lamp = &mut ctx.accounts.lava_lamp.load_init()?;
        lava_lamp.bump = bump;
        lava_lamp.schema = 1;
        lava_lamp.byte_cost = params.byte_cost;
        lava_lamp.max_size = params.buffer_size;
        lava_lamp.authority = ctx.accounts.authority.key();
        lava_lamp.buffer = ctx.accounts.buffer.key();
        lava_lamp.reward_address = ctx.accounts.reward_address.to_account_info().key();

        let buffer = &mut ctx.accounts.buffer.load_init()?;

        Ok(())
    }
}
