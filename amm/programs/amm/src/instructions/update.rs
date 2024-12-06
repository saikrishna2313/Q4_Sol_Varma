use crate::state::config::Configs;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config", config.seeds.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Configs>,
}

impl<'info> Update<'info> {
    pub fn lock(&mut self) -> Result<()> {
        self.config.locked = true;
        Ok(())
    }

    pub fn un_lock(&mut self) -> Result<()> {
        self.config.locked = false;
        Ok(())
    }
}