use anchor_lang::prelude::*;

declare_id!("B4tniotobWXRstYEAQHWX8RetNmBcMvKY1CFBkBN7pc2");

#[program]
pub mod basic_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
