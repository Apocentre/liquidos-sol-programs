use anchor_lang::prelude::*;

declare_id!("CmccctV39SQpEiVsK3hgRo6i6QW55pLBTSsEmDLw9AXY");

#[program]
pub mod onlybags_locker {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
