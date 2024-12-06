use anchor_lang::prelude::*;

use crate::{assert_non_zero, assert_not_expired, assert_not_locked};
use crate::{error::AmmError, state::config::Configs};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        associated_token::mint = mint_a,
        associated_token::authority = auth,
    )]
    pub vault_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        associated_token::mint = mint_b,
        associated_token::authority = auth,
    )]
    pub vault_b: Box<InterfaceAccount<'info, TokenAccount>>,

    ///CHECK This is okay
    #[account(
        seeds = [b"auth"],
        bump
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"mint_lp",config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = auth
    )]
    pub mint_lp: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_a,
        associated_token::authority = user
    )]
    pub user_account_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_b,
        associated_token::authority = user
    )]
    pub user_account_b: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        seeds = [b"config", config.seeds.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Configs>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Swap<'info> {
    pub fn swap(&mut self, is_a: bool, amount: u64, min_amt: u64, expiration: i64) -> Result<()> {
        assert_non_zero!([amount, min_amt]);
        assert_not_expired!(expiration);
        assert_not_locked!(self.config.locked);

        let mut curve = ConstantProduct::init(
            self.vault_a.amount,
            self.vault_b.amount,
            self.mint_lp.supply,
            self.config.fee,
            None,
        )
        .map_err(AmmError::from)?;

        let p = if is_a {
            LiquidityPair::X
        } else {
            LiquidityPair::Y
        };

        let res = curve.swap(p, amount, min_amt).map_err(AmmError::from)?;

        assert_non_zero!([res.deposit, res.withdraw]);

        self.deposit_token(is_a, res.deposit)?;
        self.withdraw(is_a, res.withdraw)
    }

    pub fn deposit_token(&mut self, is_a: bool, amount: u64) -> Result<()> {
        let mint;

        let (from, to) = if is_a {
            mint = self.mint_a.clone();
            (
                self.user_account_a.to_account_info(),
                self.vault_a.to_account_info(),
            )
        } else {
            mint = self.mint_b.clone();
            (
                self.user_account_b.to_account_info(),
                self.vault_b.to_account_info(),
            )
        };

        let cpi_accounts = TransferChecked {
            authority: self.user.to_account_info(),
            from,
            mint: mint.to_account_info(),
            to,
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(cpi_context, amount, 6)
    }

    pub fn withdraw(&mut self, is_a: bool, amount: u64) -> Result<()> {
        let mint;

        let (from, to) = if is_a {
            mint = self.mint_b.clone();

            (
                self.vault_b.to_account_info(),
                self.user_account_b.to_account_info(),
            )
        } else {
            mint = self.mint_a.clone();

            (
                self.vault_a.to_account_info(),
                self.user_account_a.to_account_info(),
            )
        };

        let seeds = &[&b"auth"[..], &[self.config.auth_bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = TransferChecked {
            from,
            to,
            mint: mint.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        transfer_checked(cpi_context, amount, 6)
    }
}