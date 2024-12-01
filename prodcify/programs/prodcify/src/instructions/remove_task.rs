use anchor_lang::{prelude::*, system_program::{Transfer, transfer}, solana_program::clock};
use crate::states::{Profile, Project,Task, TeamMember};

#[derive(Accounts)]
#[instruction(name: String, member: Pubkey, project_name: String)]
pub struct RemoveTaskContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"task", name.as_bytes(), signer.key().as_ref(), member.as_ref(), project_name.as_bytes()],
        bump = task.bump,
        close = signer
    )]
    pub task: Account<'info, Task>,

    #[account(
        mut,
        seeds = [b"vaultA", task.key().as_ref()],
        bump = task.vault_a_bump,
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
        seeds = [b"project", project_name.as_bytes()],
        bump
    )]
    pub project: Account<'info, Project>,

    #[account(
        mut,
        seeds = [b"team_member", project_name.as_bytes(), member.as_ref()],
        bump = team_member.bump
    )]
    pub team_member: Account<'info, TeamMember>,

    #[account(
        seeds = [b"profile", team_member.id.as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,
    #[account(
        mut,
        seeds = [b"pot", profile.id.as_ref(), profile.key().as_ref()],
        bump = profile.pot_bump
    )]
    pub pot: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> RemoveTaskContext<'info> {
    pub fn delete_task(&mut self) -> Result<()> {
        let project_account = &mut self.project;
        let vault_a_bump = self.task.vault_a_bump;
        let vault_b_bump = self.task.vault_b_bump;
        let task_main_subtasks = self.task.main_subtasks;
        let task_main_subtasks_completed = self.task.main_subtasks_completed;
         
        if project_account.owner != self.signer.key() || self.task.owner != self.signer.key() {
            return Err(ErrorCode::Unauthorized.into());
        }
        
        let timestamp: u64 = clock::Clock::get()?.unix_timestamp as u64;

        if timestamp > self.task.deadline{
            if task_main_subtasks == task_main_subtasks_completed {
                let binding = self.task.key();
                let seeds: &[&[u8]] = &[b"vaultA", binding.as_ref(), &[vault_a_bump]];
                let signer_seeds: &[&[&[u8]]] = &[seeds];
        
                let cpi_context = CpiContext::new_with_signer(
                    self.system_program.to_account_info(),
                    Transfer {
                        from: self.vault_a.to_account_info(),
                        to: self.signer.to_account_info(),
                    },
                    signer_seeds,
                );
                transfer(cpi_context, self.vault_a.lamports())?;

                let binding1 = self.vault_a.key();
                let seeds1: &[&[u8]] = &[b"vaultB", binding1.as_ref(), &[vault_b_bump]];
                let signer_seeds1: &[&[&[u8]]] = &[seeds1];
        
                let cpi_context1 = CpiContext::new_with_signer(
                    self.system_program.to_account_info(),
                    Transfer {
                        from:self.vault_b.to_account_info(),
                        to: self.pot.to_account_info(),
                    },
                    signer_seeds1,
                );
                transfer(cpi_context1, self.vault_b.lamports())?;
            
            } else {
                let binding = self.task.key();
                let seeds: &[&[u8]] = &[b"vaultA", binding.as_ref(), &[vault_a_bump]];
                let signer_seeds: &[&[&[u8]]] = &[seeds];
        
                let cpi_context = CpiContext::new_with_signer(
                    self.system_program.to_account_info(),
                    Transfer {
                        from: self.vault_a.to_account_info(),
                        to: self.signer.to_account_info(),
                    },
                    signer_seeds,
                );
                transfer(cpi_context, self.vault_a.lamports())?;

                let binding1 = self.vault_a.key();
                let seeds1: &[&[u8]] = &[b"vaultB", binding1.as_ref(), &[vault_b_bump]];
                let signer_seeds1: &[&[&[u8]]] = &[seeds1];
        
                let cpi_context1 = CpiContext::new_with_signer(
                    self.system_program.to_account_info(),
                    Transfer {
                        from:self.vault_b.to_account_info(),
                        to: self.signer.to_account_info(),
                    },
                    signer_seeds1,
                );
                transfer(cpi_context1, self.vault_b.lamports())?;
            }
        } else {
            
            if self.task.main_subtasks!=self.task.main_subtasks_completed{
                return Err(ErrorCode::CantDelete.into());
            }

            let binding = self.task.key();
            let seeds: &[&[u8]] = &[b"vaultA", binding.as_ref(), &[vault_a_bump]];
            let signer_seeds: &[&[&[u8]]] = &[seeds];
    
            let cpi_context = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.vault_a.to_account_info(),
                    to: self.signer.to_account_info(),
                },
                signer_seeds,
            );
            transfer(cpi_context, self.vault_a.lamports())?;

            let binding1 = self.vault_a.key();
            let seeds1: &[&[u8]] = &[b"vaultB", binding1.as_ref(), &[vault_b_bump]];
            let signer_seeds1: &[&[&[u8]]] = &[seeds1];
    
            let cpi_context1 = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                Transfer {
                    from:self.vault_b.to_account_info(),
                    to: self.pot.to_account_info(),
                },
                signer_seeds1,
            );
            transfer(cpi_context1, self.vault_b.lamports())?;
        }
        msg!("Task successfully deleted and funds transferred.");
        Ok(())
    }

    

    
}


#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: Only the project owner can delete tasks.")]
    Unauthorized,
    #[msg("The specified member does not exist in the project team.")]
    MemberNotFound,
    #[msg("You cant deleted a task, wait for deadline or main tasks need to be finished")]
    CantDelete
}
