use anchor_lang::prelude::*;
use anchor_lang::solana_program;

declare_id!("BZ7ambpq5TN49KczyL7QEfyr2xmnz1ubAHqdSC5ywd5f");

pub const WEATHER_SEED: &[u8] = b"WEATHERREPORT";

#[program]
pub mod basic_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let station_bump = ctx.bumps.get("station").unwrap().clone();

        let station = &mut ctx.accounts.station.load_init()?;
        station.bump = station_bump;
        station.schema = 1;
        station.authority = ctx.accounts.authority.key().clone();

        Ok(())
    }

    pub fn report(ctx: Context<Report>, params: ReportParams) -> Result<()> {
        let station = &mut ctx.accounts.station.load_mut()?;
        station.reports = params.reports.clone();
        station.last_updated = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

// Used to derive the location in the slice.
// We control this so we can cut corners.
pub enum Location {
    NewYork,
    London,
    Berlin,
    Tokyo,
}

#[zero_copy]
#[derive(Debug, Default, AnchorSerialize, AnchorDeserialize)]
pub struct WeatherReport {
    pub temperature: f64,
    pub windspeed: f64,
    pub weathercode: u64,
    pub timestamp: u64,
}

#[repr(packed)]
#[account(zero_copy(unsafe))]
pub struct WeatherStation {
    pub bump: u8,
    pub schema: u8,
    pub authority: Pubkey,
    pub last_updated: i64,
    pub reports: [WeatherReport; 4],
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        space = 8 + std::mem::size_of::<WeatherStation>(),
        payer = payer,
        seeds = [WEATHER_SEED],
        bump
    )]
    pub station: AccountLoader<'info, WeatherStation>,
    /// CHECK:
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,
    /// CHECK:
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: AccountInfo<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ReportParams {
    pub reports: [WeatherReport; 4],
}

#[derive(Accounts)]
#[instruction(params: ReportParams)] // rpc parameters hint
pub struct Report<'info> {
    #[account(
        mut,
        seeds = [WEATHER_SEED],
        bump = station.load()?.bump
    )]
    pub station: AccountLoader<'info, WeatherStation>,
    // TODO: Add oracle and node account
}
