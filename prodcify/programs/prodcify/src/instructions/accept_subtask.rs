use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL, system_program::{transfer, Transfer}};
use crate::states::{Project, Status, Subtask, Task};

#[derive(Accounts)]
#[instruction(name: String, member: Pubkey, project_name: String, task_name: String)]
pub struct AcceptSubtaskContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"task", task_name.as_bytes(), signer.key().as_ref(), member.as_ref(), project_name.as_bytes()],
        bump = task.bump
    )]
    pub task: Account<'info, Task>,
    #[account(
        mut,
        seeds = [b"subtask", task.key().as_ref(),name.as_bytes()],
        bump = subtask.bump
    )]
    pub subtask: Account<'info, Subtask>,
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
    pub system_program: Program<'info, System>,
}

impl<'info> AcceptSubtaskContext<'info> {
    pub fn accept_subtask(&mut self) -> Result<()> {
        let project_account = &mut self.project;
        let task = &mut self.task;
        let subtask = &mut self.subtask;

        if project_account.owner != self.signer.key() {
            return Err(ErrorCode::Unauthorized.into());
        }
        if task.owner != self.signer.key() {
            return Err(ErrorCode::Unauthorized.into());
        }
        if subtask.owner != self.signer.key() {
            return Err(ErrorCode::MemberNotFound.into());
        }

        let ctx_accounts = Transfer {
            from:  self.vault_a.to_account_info(),
            to: self.vault_b.to_account_info(),
        };

        let binding = task.key();
        let seeds: &[&[u8]] = &[b"vaultA", binding.as_ref(), &[task.vault_a_bump]];
        let signer_seeds: &[&[&[u8]]] = &[&seeds];

        let cpi_context = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            ctx_accounts,
            signer_seeds,
        );
        transfer(cpi_context, subtask.reward*LAMPORTS_PER_SOL)?;
        subtask.status = Status::Completed;
        task.main_subtasks_completed+=1;
        task.completed_subtasks+=1;
        task.points+=subtask.points;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: Only the project owner can perform this action.")]
    Unauthorized,
    #[msg("The specified member does not exist in the project team.")]
    MemberNotFound,
}
