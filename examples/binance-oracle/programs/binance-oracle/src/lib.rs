use anchor_lang::prelude::*;

declare_id!("4ih7pcGkVT2HBJuXqTFemhyd73BQktBkuKXyrbRZn22v");

#[program]
pub mod binance_oracle {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
