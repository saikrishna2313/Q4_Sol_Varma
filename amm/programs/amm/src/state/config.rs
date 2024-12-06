use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
pub struct Configs {
    pub seeds: u64,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub config_bump: u8,
    pub auth_bump: u8,
    pub fee: u16,
    pub locked: bool,
    pub authority: Pubkey
}

impl Configs {
    // Using variables so that there are no magic numbers
    pub const LEN: usize = 3 * PUBKEY_L + 1 * U16_L + 1 * BOOL_L + 1 * OPTION_L + 2 * U8_L + 8;
}