use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::states::{Profile, Project, Task, TeamMember};

#[derive(Accounts)]
#[instruction(name: String, project_name: String)]
pub struct MemberExitTask<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"project", project_name.as_bytes()],
        bump=project.bump
    )]
    pub project: Account<'info, Project>,
    #[account(
        mut,
        seeds = [b"task", name.as_bytes(), project.owner.as_ref(), signer.key().as_ref(), project_name.as_bytes()],
        bump = task.bump,
    )]
    pub task: Account<'info, Task>,
    
    #[account(
        mut,
        seeds = [b"vaultA", task.key().as_ref()],
        bump = task.vault_a_bump
    )]
    pub vault_a: SystemAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"vaultB", vault_a.key().as_ref()],
        bump = task.vault_b_bump
    )]
    pub vault_b: SystemAccount<'info>,


    #[account(
        mut,
        seeds = [b"team_member", project_name.as_bytes(), signer.key().as_ref()],
        bump = team_member.bump
    )]
    pub team_member: Account<'info, TeamMember>,

    #[account(
        mut,
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

impl<'info> MemberExitTask<'info> {
    pub fn exit_task(&mut self) -> Result<()> {
        let task = &mut self.task;
        let signer_key = self.signer.key();
       

        if task.member != signer_key {
            return Err(ErrorCode::Unauthorized.into());
        }
          
        if task.main_subtasks_completed != task.main_subtasks {
            let ctx_accounts = Transfer {
                from:  self.vault_b.to_account_info(),
                to: self.vault_a.to_account_info(),
            };
    
            let binding = self.vault_a.key();
            let seeds: &[&[u8]] = &[b"vaultB", binding.as_ref(), &[task.vault_b_bump]];
            let signer_seeds: &[&[&[u8]]] = &[&seeds];
    
            let cpi_context = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                ctx_accounts,
                signer_seeds,
            );
            transfer(cpi_context,self.vault_b.lamports())?;
        }else{
            let ctx_accounts = Transfer {
                from:  self.vault_b.to_account_info(),
                to: self.pot.to_account_info(),
            };
    
            let binding =self.vault_a.key();
            let seeds: &[&[u8]] = &[b"vaultB", binding.as_ref(), &[task.vault_b_bump]];
            let signer_seeds: &[&[&[u8]]] = &[&seeds];
    
            let cpi_context = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                ctx_accounts,
                signer_seeds,
            );
            transfer(cpi_context,self.vault_b.lamports())?;
        }
        self.team_member.incompleted_tasks+=1;
        self.profile.tasks_incompleted+=1;
        Ok(())

    }

   
}
#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: Only the task owner can exit.")]
    Unauthorized,
    #[msg("The specified member does not exist in the project team.")]
    MemberNotFound,
}