use crate::*;

#[derive(Accounts)]
#[instruction(params: ReadRandomnessParams)] // rpc parameters hint
pub struct ReadRandomness<'info> {
    #[account(
        mut,
        seeds = [b"LAVALAMPSTATE"], 
        bump = lava_lamp.load()?.bump,
        has_one = buffer,
        // has_one = reward_address,
    )]
    pub lava_lamp: AccountLoader<'info, LavaLampAccount>,

    #[account(mut)]
    pub buffer: AccountLoader<'info, BufferAccount>,
    // #[account(mut)]
    // pub reward_address: Box<Account<'info, TokenAccount>>,

    // #[account(
    //     mut,
    //     constraint = token_payer.mint == reward_address.mint
    // )]
    // pub token_payer: Box<Account<'info, TokenAccount>>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ReadRandomnessParams {
    pub num_bytes: u32,
}

impl ReadRandomness<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, params: &ReadRandomnessParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: &Context<Self>, params: &ReadRandomnessParams) -> Result<()> {
        // transfer tokens

        // update read idx
        let lava_lamp = &mut ctx.accounts.lava_lamp.load_mut()?;
        let new_read_idx = lava_lamp.read_idx + params.num_bytes % lava_lamp.max_size;
        lava_lamp.read_idx = new_read_idx;

        Ok(())
    }
}
