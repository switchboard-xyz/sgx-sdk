use crate::*;

#[derive(Accounts)]
#[instruction(params: BubbleParams)] // rpc parameters hint
pub struct Bubble<'info> {
    #[account(
        // READ ONLY, write lock protection
        seeds = [b"LAVALAMPSTATE"], 
        bump = lava_lamp.load()?.bump,
        has_one = buffer
    )]
    pub lava_lamp: AccountLoader<'info, LavaLampAccount>,

    #[account(mut)]
    pub buffer: AccountLoader<'info, BufferAccount>,

    pub function: AccountLoader<'info, FunctionAccountData>,

    #[account(
        has_one = secured_signer @ Meltdown::InvalidTrustedSigner
    )]
    pub quote: AccountLoader<'info, QuoteAccountData>,

    pub secured_signer: Signer<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BubbleParams {
    pub data: Vec<u8>,
}

impl Bubble<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, params: &BubbleParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: &Context<Self>, params: &BubbleParams) -> Result<()> {
        let lava_lamp = ctx.accounts.lava_lamp.load()?;

        let function = ctx.accounts.function.load()?;

        let quote = ctx.accounts.quote.load()?;

        // TODO: handle round robin
        let buffer_account_info = ctx.accounts.buffer.to_account_info();
        let buffer_data: &mut [u8] = &mut buffer_account_info.try_borrow_mut_data()?[8..];

        let mut write_idx_bytes: [u8; 4] = [0u8; 4];
        write_idx_bytes.copy_from_slice(&buffer_data[0..4]);
        let write_idx = u32::from_le_bytes(write_idx_bytes);

        let new_write_idx = write_idx + params.data.len() as u32;

        let buffer = &mut buffer_data[(4 + write_idx as usize)..(4 + new_write_idx as usize)];
        buffer.copy_from_slice(&params.data[..]);

        // update write idx
        buffer_data[..4].copy_from_slice(&(new_write_idx).to_le_bytes());

        Ok(())
    }
}
