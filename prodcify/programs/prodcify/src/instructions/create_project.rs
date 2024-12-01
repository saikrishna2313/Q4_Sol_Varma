use anchor_lang::{prelude::*, solana_program::clock};
use crate::states::Project;


#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateProjectContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = Project::SPACE,
        seeds = [b"project", name.as_bytes()],
        bump
    )]
    pub project: Account<'info, Project>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateProjectContext<'info> {
    pub fn initialize_project(&mut self, name: String, bumps:&CreateProjectContextBumps) -> Result<()> {
        let project = &mut self.project;
        project.name = name;
        project.owner = self.signer.key();
        project.created = clock::Clock::get()?.unix_timestamp as u64;
        project.bump=bumps.project;
        Ok(())
    }
}
