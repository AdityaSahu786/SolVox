use anchor_lang::prelude::*;

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(ctx: Context<InitializePoll>, _poll_id: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Poll {
    pub poll_id: u64,
    pub votes: u64,
}
