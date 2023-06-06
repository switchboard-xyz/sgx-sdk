use anchor_lang::solana_program::{instruction::Instruction, program::invoke};
use lava_lamp::{program::LavaLamp, BufferAccount, LavaLampAccount, ReadRandomness};

use crate::*;

#[derive(Accounts)]
#[instruction(params: ReadFluxParams)] // rpc parameters hint
pub struct ReadFlux<'info> {
    #[account(
        mut,
        seeds = [b"FLUXSTATE"], 
        bump = flux_state.load()?.bump
    )]
    pub flux_state: AccountLoader<'info, FluxAccount>,

    #[account(
        has_one = buffer
    )]
    pub lava_lamp: AccountLoader<'info, LavaLampAccount>,
    pub buffer: AccountLoader<'info, BufferAccount>,
    pub lava_lamp_program: Program<'info, LavaLamp>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ReadFluxParams {}

impl ReadFlux<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, params: &ReadFluxParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: &Context<Self>, params: &ReadFluxParams) -> Result<()> {
        let flux = &mut ctx.accounts.flux_state.load_mut()?;

        // first invoke CPI to get the read idx
        {
            let read_lava_lamp_accounts = ReadRandomness {
                lava_lamp: ctx.accounts.lava_lamp.clone(),
                buffer: ctx.accounts.buffer.clone(),
            };
            let mut data: Vec<u8> =
                lava_lamp::instruction::ReadRandomness::discriminator().try_to_vec()?;
            let mut num_bytes = u64::to_le_bytes(8).try_to_vec()?;
            data.append(&mut num_bytes);

            let read_ixn = Instruction::new_with_bytes(
                ctx.accounts.lava_lamp_program.key(),
                &data,
                read_lava_lamp_accounts.to_account_metas(None),
            );

            invoke(&read_ixn, &read_lava_lamp_accounts.to_account_infos())?;
        }

        // then read the buffer using the current read_idx as the ending slice
        {
            let lava_lamp = ctx.accounts.lava_lamp.load()?;

            let buffer_account_info = ctx.accounts.buffer.to_account_info();
            let buffer_data: &mut [u8] = &mut buffer_account_info.try_borrow_mut_data()?[12..];

            let mut value_bytes: [u8; 8] = [0u8; 8];
            value_bytes.copy_from_slice(
                &buffer_data[(lava_lamp.read_idx as usize - 8)..(lava_lamp.read_idx as usize)],
            );
            let value = u64::from_le_bytes(value_bytes);
            flux.value = value;
        }

        Ok(())
    }
}
