use anchor_lang::{prelude::*, solana_program::clock};
use crate::states::{Profile, TeamMember};

#[derive(Accounts)]
#[instruction(name: String, project_name: String, member: Pubkey)]
pub struct CreateTeamMemberContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = TeamMember::SPACE,
        seeds = [b"team_member", project_name.as_bytes(), member.key().as_ref()],
        bump
    )]
    pub team_member: Account<'info, TeamMember>,

    #[account(
        mut,
        seeds = [b"profile", member.as_ref()],
        bump=profile.bump
    )]
    pub profile: Account<'info, Profile>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateTeamMemberContext<'info> {
    pub fn initialize_team_member(
        &mut self,
        name: String,
        project_name: String,
        member:Pubkey,
        bumps: &CreateTeamMemberContextBumps,
    ) -> Result<()> {
        let team_member = &mut self.team_member;
        team_member.name = name;
        team_member.id =member;
        team_member.project = project_name.clone();
        team_member.completed_tasks = 0;
        team_member.pending_tasks = 0;
        team_member.incompleted_tasks = 0;
        team_member.joining_date = clock::Clock::get()?.unix_timestamp as u64;
        team_member.bump = bumps.team_member;
        self.profile.projects+=1;

        Ok(())
    }
}

