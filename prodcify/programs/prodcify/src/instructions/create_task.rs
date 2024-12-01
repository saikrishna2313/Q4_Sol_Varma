use core::time;

use anchor_lang::prelude::*;
use crate::states::{Project, Status, Task};
#[derive(Accounts)]
#[instruction(name: String, member: Pubkey, project_name: String, deadline: u64)]
pub struct CreateTaskContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = Task::SPACE,
        seeds = [b"task",name.as_bytes(),signer.key().as_ref(), member.as_ref(), project_name.as_bytes()],
        bump
    )]
    pub task: Account<'info, Task>,
 
    #[account(
        seeds = [b"vaultA",task.key().as_ref()],
        bump
    )]
    pub vault_a: SystemAccount<'info>,
 
    #[account(
        seeds = [b"vaultB",vault_a.key().as_ref() ],
        bump
    )]
    pub vault_b: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"project", project_name.as_bytes()],
        bump=project.bump
    )]
    pub project: Account<'info, Project>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateTaskContext<'info> {
    pub fn initialize_new_task(
        &mut self,
        name: String,
        project_name: String,
        member:Pubkey,
        deadline: u64,
        bumps:&CreateTaskContextBumps,
    ) -> Result<()> {
        let project_account = &mut self.project;
        if project_account.owner != self.signer.key() {
            return Err(ErrorCode::Unauthorized.into());
        }
        
        let timestamp:u64=Clock::get()?.unix_timestamp as u64;
        if timestamp>deadline {
            return Err(ErrorCode::OutdatedDeadline.into());
        }else{
            msg!("Creating Task");
        }
        let task = &mut self.task;
        task.name = name;
        task.owner=self.signer.key();
        task.member=member;
        task.project = project_name.clone();
        task.subtasks =Vec::new();
        task.completed_subtasks = 0;
        task.main_subtasks=0;
        task.main_subtasks_completed=0;
        task.total_subtasks =0;
        task.start_time = Clock::get()?.unix_timestamp as u64;
        task.deadline = deadline;
        task.accepted = false;
        task.bump = bumps.task;
        task.vault_a=self.vault_a.key();
        task.vault_b=self.vault_b.key();
        task.vault_a_bump=bumps.vault_a;
        task.vault_b_bump=bumps.vault_b;
        task.total_reward=0;
        task.status=Status::Pending;
            
        Ok(())
    }
}



#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: Only the project owner can perform this action.")]
    Unauthorized,
    #[msg("Outdated Deadline Please Check Again")]
    OutdatedDeadline
}
