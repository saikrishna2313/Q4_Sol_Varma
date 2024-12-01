use anchor_lang::prelude::*;

use super::Status;


#[account]
pub struct Subtask{
    pub name: String,
    pub project:String,
    pub task:Pubkey,
    pub owner:Pubkey,
    pub member:Pubkey,
    pub reward:u64,
    pub task_type:Type,
    pub bump:u8,
    pub points:u64,
    pub status:Status
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum Type {
       Main,
       Bonus,
}

impl Subtask {
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 32 + 8 + 1 + 1 + 1;
}