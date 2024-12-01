use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::states::{Project, Status, Task, TeamMember, Profile};

#[derive(Accounts)]
#[instruction(name: String, member: Pubkey, project_name: String)]
pub struct AcceptTaskContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"task", name.as_bytes(), signer.key().as_ref(), member.as_ref(), project_name.as_bytes()],
        bump = task.bump
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
        seeds = [b"project", project_name.as_bytes()],
        bump = project.bump
    )]
    pub project: Account<'info, Project>,

    #[account(
        mut,
        seeds = [b"team_member", project_name.as_bytes(), member.as_ref()],
        bump = team_member.bump
    )]
    pub team_member: Account<'info, TeamMember>,

    #[account(
        mut,
        seeds = [b"profile", member.as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(
        mut,
        seeds = [b"pot", member.as_ref(), profile.key().as_ref()],
        bump=profile.pot_bump
    )]
    pub pot: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> AcceptTaskContext<'info> {
    pub fn accept_task(&mut self) -> Result<()> {
        let project_account = &mut self.project;
        let task = &mut self.task;
        let profile = &mut self.profile;
        let member_pot = &mut self.pot;
        
        let present=Clock::get()?.unix_timestamp as u64;
        let deadline=task.deadline;
        if present<deadline && task.main_subtasks_completed != task.main_subtasks{
            return Err(ErrorCode::PendingTasksLeft.into());
        }
        if project_account.owner != self.signer.key() {
            return Err(ErrorCode::Unauthorized.into());
        }
        if task.owner != self.signer.key() {
            return Err(ErrorCode::Unauthorized.into());
        }

        let cpi_accounts = Transfer {
            from: self.vault_a.to_account_info(),
            to: self.signer.to_account_info(),
        };

        let binding = task.key();
        let seeds = &[b"vaultA", binding.as_ref(), &[task.vault_a_bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        transfer(cpi_context, self.vault_a.lamports())?;

       
        let cpi_accounts2 = Transfer {
            from: self.vault_b.to_account_info(),
            to: member_pot.to_account_info(),
        };
    let binding2 = self.vault_a.key();
        
        let seeds2 = &[b"vaultB", binding2.as_ref(), &[task.vault_b_bump]];
        let signer_seeds2 = &[&seeds2[..]];

        let cpi_context2 = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            cpi_accounts2,
            signer_seeds2,
        );
        transfer(cpi_context2, self.vault_b.lamports())?;

        task.status = Status::Completed;
        profile.points += task.points;
        self.team_member.completed_tasks+=1;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: Only the project owner can accept tasks.")]
    Unauthorized,
    #[msg("The specified member does not exist in the project team.")]
    MemberNotFound,
    #[msg("Still Some subtaks are left")]
    PendingTasksLeft,
}
