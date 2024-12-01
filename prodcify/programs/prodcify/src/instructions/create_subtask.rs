use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL, system_program::{transfer, Transfer}};
use crate::states::{Project, Status, Subtask, Task, Type};

#[derive(Accounts)]
#[instruction(name: String, member: Pubkey,points:u64, project_name: String, reward: u64, task_name: String,main:bool)]
pub struct CreateSubTaskContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"task", task_name.as_bytes(), signer.key().as_ref(), member.key().as_ref(), project_name.as_bytes()],
        bump=task.bump
    )]
    pub task: Account<'info, Task>,
    #[account(
        init,
        payer = signer,
        space = Subtask::SPACE,
        seeds = [b"subtask",task.key().as_ref(),name.as_bytes()],
        bump
    )]
    pub subtask: Account<'info, Subtask>,
    #[account(
        mut,
        seeds = [b"vaultA", task.key().as_ref()],
        bump=task.vault_a_bump
    )]
    pub vault_a: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"project", project_name.as_bytes()],
        bump=project.bump
    )]
    pub project: Account<'info, Project>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateSubTaskContext<'info> {
    pub fn initialize_new_subtask(
        &mut self,
        points:u64,
        name: String,
        reward: u64,
        main:bool,
        bumps:&CreateSubTaskContextBumps
    ) -> Result<()> {
        let project_account = &mut self.project;
        let task = &mut self.task;
        let subtask = &mut self.subtask;

        if project_account.owner != self.signer.key() {
            return Err(ErrorCode::Unauthorized.into());
        }
        if task.owner != self.signer.key() {
            return Err(ErrorCode::Unauthorized.into());
        
        }
        subtask.name = name.clone();
        subtask.project =project_account.name.clone();
        subtask.reward = reward;
        subtask.bump = bumps.subtask;
        subtask.status = Status::Pending;
        if main==true{
            subtask.task_type = Type::Main;
        }else{
            subtask.task_type=Type::Bonus;
        }
        subtask.owner = self.signer.key();
        subtask.member = task.member;
        subtask.points=points;
        let cpi_program = self.system_program.to_account_info();

  
        let cpi_accounts = Transfer {
            from: self.signer.to_account_info(),       
            to: self.vault_a.to_account_info(),       
        };
          
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, reward*LAMPORTS_PER_SOL)?;
        task.subtasks.push(name);
        if main==true {
            task.main_subtasks+=1;
        }
        task.total_subtasks+=1;
        task.total_reward=task.total_reward+reward;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: Only the project owner can create tasks.")]
    Unauthorized,
    #[msg("The specified member does not exist in the project team.")]
    MemberNotFound,
}
