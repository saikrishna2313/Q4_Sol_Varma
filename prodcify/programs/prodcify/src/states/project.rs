use anchor_lang::prelude::*;

#[account]
pub struct Project {
    pub name: String,
    pub created: u64,
    pub owner: Pubkey,
    pub bump: u8,
}

impl Project {
    pub const SPACE: usize = 8 + 4 + 30 + 8 + 32 + 1;
}
