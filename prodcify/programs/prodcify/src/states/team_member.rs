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
    pub const SPACE: usize = 8 + // Account discriminator (8 bytes)
        4 + 35 + // Name: 4 bytes for length + 35 bytes for string data
        4 + 32 + // Project: 4 bytes for length + 35 bytes for string data
        32 + // Pubkey (id) (32 bytes)
        8 + // Completed tasks count (8 bytes)
        8 + // Pending tasks count (8 bytes)
        8 + // Incompleted tasks count (8 bytes)
        1 + // Bump (1 byte)
        8; // Joining date (8 bytes)
}