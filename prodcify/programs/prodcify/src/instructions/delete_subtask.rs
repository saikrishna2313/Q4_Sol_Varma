use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::states::{Status, Subtask, Task};

#[derive(Accounts)]
#[instruction(name: String, member: Pubkey, project_name: String,task_name:String)]
pub struct RemoveSubtaskContext<'info> {
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
        seeds = [b"subtask",task.key().as_ref(),task_name.as_bytes()],
        bump=subtask.bump,
        close=signer
    )]
    pub subtask: Account<'info, Subtask>,

    pub system_program: Program<'info, System>,
}

impl<'info> RemoveSubtaskContext<'info> {
    pub fn delete_sub_task(&mut self) -> Result<()> {
        let subtask=&mut self.subtask;     

        if subtask.status== Status::Completed {
         return Err(ErrorCode::CantDelete.into());
        }
        let vault_a_bump = self.task.vault_a_bump;
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

        msg!("Subtask successfully deleted and funds transferred.");
        Ok(())
    }


}


#[error_code]
pub enum ErrorCode {
    #[msg("You cant deleted a task, since you are already accepted")]
    CantDelete
}
