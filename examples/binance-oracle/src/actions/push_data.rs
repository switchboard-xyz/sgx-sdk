use switchboard_solana::{FunctionAccountData, QuoteAccountData};

use crate::*;

#[derive(Accounts)]
#[instruction(params: PushDataParams)] // rpc parameters hint
pub struct PushData<'info> {
    #[account(
        seeds = [PROGRAM_SEED],
        bump = state.load()?.bump
    )]
    pub state: AccountLoader<'info, ProgramState>,

    #[account(
        init_if_needed,
        space = 8 + std::mem::size_of::<OracleState>(),
        payer = payer,
        seeds = [state.key().as_ref(), &params.symbol[..]],
        bump
    )]
    pub oracle: AccountLoader<'info, OracleState>,

    pub function: AccountLoader<'info, FunctionAccountData>,

    #[account(
        has_one = secured_signer @ BinanceOracleError::InvalidTrustedSigner,
        constraint = state.load()?.mr_enclaves.contains(&quote.load()?.mr_enclave) @ BinanceOracleError::InvalidMrEnclave
    )]
    pub quote: AccountLoader<'info, QuoteAccountData>,

    pub secured_signer: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,

    /// CHECK:
    #[account(address = anchor_lang::solana_program::sysvar::rent::ID)]
    pub rent: AccountInfo<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct PushDataParams {
    pub symbol: [u8; 32],
    pub price: f64,
    pub volume: f64,
    pub oracle_timestamp: i64,
}

impl PushData<'_> {
    pub fn validate(_ctx: &Context<Self>, _params: &PushDataParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: &Context<Self>, params: &PushDataParams) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle.load_mut()?;

        if oracle.bump == 0 {
            oracle.symbol = params.symbol;
            oracle.bump = *ctx.bumps.get("oracle").unwrap();
        }

        if oracle.oracle_timestamp != 0 && oracle.oracle_timestamp > params.oracle_timestamp {
            return Err(error!(BinanceOracleError::StaleData));
        }

        oracle.price = params.price;
        oracle.volume = params.volume;
        oracle.oracle_timestamp = params.oracle_timestamp;
        oracle.timestamp = anchor_lang::solana_program::clock::Clock::get()?.unix_timestamp;

        Ok(())
    }
}
