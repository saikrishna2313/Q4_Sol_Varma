use anchor_lang::prelude::*;

#[account]
pub struct Task {
    pub name: String,
    pub project: String,
    pub accepted: bool,
    pub bump: u8,
    pub status: Status,
    pub owner: Pubkey,

    pub member: Pubkey,

    pub completed_subtasks: u64,
    pub total_subtasks: u64,
    pub main_subtasks: u64,
    pub main_subtasks_completed: u64,
    pub start_time: u64,
    pub deadline: u64,
    pub vault_a: Pubkey,

    pub vault_b: Pubkey,

    pub vault_a_bump: u8,
    pub vault_b_bump: u8,
    pub subtasks: Vec<String>,
    pub points: u64,
    pub total_reward: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum Status {
    Completed,
    Pending,
}

impl Task {
    pub const MAX_NAME_LEN: usize = 32; // Max length of task name
    pub const MAX_PROJECT_LEN: usize = 32; // Max length of project name
    pub const MAX_SUBTASKS: usize = 30; // Max number of subtasks
    pub const SPACE: usize = 8               // Discriminator (8 bytes)
        + (4 + Self::MAX_NAME_LEN)           // Name field (length + max chars)
        + (4 + Self::MAX_PROJECT_LEN)        // Project field (length + max chars)
        + 1                                 // Accepted (bool)
        + 1                                 // Bump (u8)
        + 1                                 // Status (enum with 2 variants)
        + 32                                // Owner (Pubkey)
        + 32                                // Member (Pubkey)
        + 8                                 // Completed subtasks (u64)
        + 8                                 // Total subtasks (u64)
        + 8                                 // Main subtasks (u64)
        + 8                                 // Main subtasks completed (u64)
        + 8                                 // Start time (u64)
        + 8                                 // Deadline (u64)
        + 32                                // Vault A (Pubkey)
        + 32                                // Vault B (Pubkey)
        + 1                                 // Vault A bump (u8)
        + 1                                 // Vault B bump (u8)
        + 4 + (Self::MAX_SUBTASKS * (Self::MAX_NAME_LEN + 4))  // Subtasks (vec of strings)
        + 8                                 // Points (u64)
        + 8; // Total reward (u64)
}
