use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod helper;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seeds: u64, fee: u16) -> Result<()> {
        ctx.accounts.initialize(seeds, &ctx.bumps, fee)
    }

    pub fn deposit_tokens(
        ctx: Context<Deposit>,
        amount: u64,
        max_a: u64,
        max_b: u64,
        expiration: i64,
    ) -> Result<()> {
        ctx.accounts.deposit(amount, max_a, max_b, expiration)
    }

    pub fn swap(
        ctx: Context<Swap>,
        is_a: bool,
        min_amt: u64,
        expiration: i64,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.swap(is_a, amount, min_amt, expiration)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
        min_a: u64,
        min_b: u64,
        expiration: i64,
    ) -> Result<()> {
        ctx.accounts.withdraw(amount, min_a, min_b, expiration)
    }
    pub fn update(ctx: Context<Update>, lock: bool) -> Result<()> {
        if lock {
            ctx.accounts.lock()
        } else {
            ctx.accounts.un_lock()
        }
    }
}