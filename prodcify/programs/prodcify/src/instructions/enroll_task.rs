use anchor_lang::prelude::*;
use crate::states::{Project, Task, TeamMember};

#[derive(Accounts)]
#[instruction(project_name: String, task_name: String)]
pub struct EnrollTaskContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"project", project_name.as_bytes()],
        bump = project.bump
    )]
    pub project: Account<'info, Project>,

    #[account(
        mut,
        seeds = [b"task", task_name.as_bytes(),project.owner.as_ref(), signer.key().as_ref(), project_name.as_bytes()],
        bump = task.bump
    )]
    pub task: Account<'info, Task>,
  
    #[account(
        mut,
        seeds = [b"team_member", project_name.as_bytes(), signer.key().as_ref()],
        bump = team_member.bump,
    )]
    pub team_member: Account<'info, TeamMember>,
    pub system_program: Program<'info, System>,
}

impl<'info> EnrollTaskContext<'info> {
    pub fn enroll_task(&mut self) -> Result<()> {
        let task = &mut self.task;
        let team_member = &mut self.team_member;
        if task.member != self.signer.key() {
            return Err(ErrorCode::Unauthorized.into());
        }
        team_member.pending_tasks += 1;
        task.accepted = true;

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized.")]
    Unauthorized,
    #[msg("Team member does not exist or is unauthorized.")]
    UnauthorizedTeamMember,
}
