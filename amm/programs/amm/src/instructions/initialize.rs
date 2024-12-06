use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{
        Mint,
        TokenAccount,
        TokenInterface
    },
    associated_token::AssociatedToken
};

use crate::state::config::Configs;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        associated_token::mint = mint_a,
        associated_token::authority = auth,
        payer = initializer
    )]
    pub vault_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init,
        associated_token::mint = mint_b,
        associated_token::authority = auth,
        payer = initializer
    )]
    pub vault_b: Box<InterfaceAccount<'info, TokenAccount>>,

    ///CHECK This is okay
    #[account(
        seeds = [b"auth"],
        bump
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        payer = initializer,
        space = Configs::LEN
    )]
    pub config: Account<'info, Configs>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
}


impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, seeds: u64, bumps: &InitializeBumps, fee: u16)-> Result<()> {

        self.config.set_inner(Configs {
            auth_bump: bumps.auth,
            seeds,
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            config_bump: bumps.config,
            fee,
            locked: false,
            authority: self.auth.key()
        });

        Ok(())
    }
}