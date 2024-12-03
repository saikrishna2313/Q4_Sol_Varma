use anchor_lang::prelude::*;
pub mod instructions;
pub mod states;
pub use instructions::*;


declare_id!("ESiApdjLUkyuGviqZk2DLaf9ew4GpbgpfDmZEGYrXJa");

#[program]
pub mod prodcify {

    use super::*;

    pub fn create_profile(ctx: Context<CreateProfileContext>, name: String) -> Result<()> {
        ctx.accounts.initialize_profile(name,&ctx.bumps)?;
        Ok(())
    }

    pub fn create_project(ctx: Context<CreateProjectContext>, name: String) -> Result<()> {
        ctx.accounts.initialize_project(name,&ctx.bumps)?;
        Ok(())
    }

    pub fn create_team_member(
        ctx: Context<CreateTeamMemberContext>,
        name: String,
        project_name: String,
        member:Pubkey
    ) -> Result<()> {
        ctx.accounts.initialize_team_member(name,project_name,member,&ctx.bumps)?;
        Ok(())
    }
    pub fn create_task(
        ctx: Context<CreateTaskContext>,
        name: String, member: Pubkey, project_name: String, deadline: u64
    )->Result<()>
        {
        ctx.accounts.initialize_new_task(name, project_name, member, deadline, &ctx.bumps)?;
        Ok(())

    }
    pub fn create_subtask(
        ctx: Context<CreateSubTaskContext>,
        name: String, member: Pubkey,points:u64, project_name: String, reward: u64, task_name: String, main:bool
    ) -> Result<()> {
        ctx.accounts.initialize_new_subtask(
            points,
            name,
            reward,
            main,&ctx.bumps)?;
        Ok(())
    }
    pub fn enroll_task(
        ctx: Context<EnrollTaskContext>,
        project_name: String, task_name: String
    ) -> Result<()> {
        ctx.accounts.enroll_task()?;
        Ok(())
    }

    pub fn accept_subtask(
        ctx: Context<AcceptSubtaskContext>,
        name: String, member: Pubkey, project_name: String, task_name: String
    ) -> Result<()> {
        ctx.accounts.accept_subtask()?;
        Ok(())
    }
    
    
    pub fn accept_task(
        ctx: Context<AcceptTaskContext>,
        name: String, member: Pubkey, project_name: String
    ) -> Result<()> {
        ctx.accounts.accept_task();
        Ok(())
    }
    pub fn member_exit_task(
        ctx: Context<MemberExitTask>,
        name: String, project_name: String
    ) -> Result<()> {
        ctx.accounts.exit_task();
        Ok(())
    }
    pub fn remove_task(
        ctx: Context<RemoveTaskContext>,
        name: String, member: Pubkey, project_name: String
    ) -> Result<()> {
        ctx.accounts.delete_task();
        Ok(())
    }    

    pub fn withdraw_funds(ctx:Context<WithdrawFundsContext>)->Result<()>{
       ctx.accounts.withdraw_funds();
       Ok(())
    }

  
}

