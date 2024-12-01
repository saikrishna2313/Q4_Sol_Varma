use anchor_lang::prelude::*;

#[account]
pub struct Profile {
    pub id: Pubkey,
    pub name: String,
    pub points: u64,
    pub projects: u64,
    pub tasks_completed: u64,
    pub tasks_incompleted: u64,
    pub pot: Pubkey,
    pub bump: u8,
    pub pot_bump: u8,
}

impl Profile {
    pub const SPACE: usize = 8 + 32 + 34 + 8 + 8 + 8 + 32 + 1 + 1;
}