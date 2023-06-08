use anchor_lang::prelude::*;

declare_id!("3BJKeRy4WX4YQqsGWfmfEn2LhQ81VgaiMmFb2XiY8f2H");

#[program]
pub mod coinbase_vwap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
