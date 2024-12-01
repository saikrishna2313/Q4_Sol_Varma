use anchor_lang::prelude::*;
use crate::states::Profile;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateProfileContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = Profile::SPACE,
        seeds = [b"profile", signer.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,
    #[account(
        seeds = [b"pot", signer.key().as_ref(), profile.key().as_ref()],
        bump
    )]
    pub pot: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateProfileContext<'info> {
    pub fn initialize_profile(&mut self, name: String, bumps:&CreateProfileContextBumps) -> Result<()> {
        let profile = &mut self.profile;
        
       
        profile.id = self.signer.key();  
        profile.name = name;  
        profile.points = 0;  
        profile.projects = 0;  
        profile.tasks_completed = 0;  
        profile.tasks_incompleted = 0;  
        profile.pot = self.pot.key();  
        profile.bump = bumps.profile; 
        profile.pot_bump = bumps.pot; 

        Ok(())
    }
}