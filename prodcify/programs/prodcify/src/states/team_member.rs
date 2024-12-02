use anchor_lang::prelude::*;

#[account]
pub struct TeamMember {
    pub name: String,
    pub id: Pubkey,
    pub project: String,
    pub completed_tasks: u64,
    pub pending_tasks: u64,
    pub incompleted_tasks: u64,
    pub bump: u8,
    pub joining_date: u64,
}

impl TeamMember {
    pub const SPACE: usize = 8 + 
        4 + 35 + 
        4 + 32 + 
        32 + 
        8 + 
        8 + 
        8 + 
        1 + // Bump (1 byte)
        8; // Joining date (8 bytes)
}