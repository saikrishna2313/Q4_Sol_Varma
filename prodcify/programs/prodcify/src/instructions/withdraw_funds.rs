use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::states::{Profile};

#[derive(Accounts)]
pub struct WithdrawFundsContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
     
        seeds = [b"profile", signer.key().as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(
        mut,
        seeds = [b"pot", signer.key().as_ref(), profile.key().as_ref()],
        bump = profile.pot_bump
    )]
    pub pot: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawFundsContext<'info> {
    pub fn withdraw_funds(&mut self) -> Result<()> {
     
        let binding = self.profile.key();
        let binding1 = self.signer.key();
        let seeds: &[&[u8]] = &[b"pot",binding1.as_ref(),binding.as_ref(), &[self.profile.pot_bump]];
        let signer_seeds: &[&[&[u8]]] = &[seeds];

        let cpi_context = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            Transfer {
                from:self.pot.to_account_info() ,
                to:self.signer.to_account_info(),
            },
            signer_seeds,
        );
        transfer(cpi_context,self.pot.lamports())?;
        Ok(())
        
    }

   
}
