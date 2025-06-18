use anchor_lang::prelude::*;

declare_id!("Fo5u8WAkf2H2JJe72RjwMV2ob4JeDk2shfm7kfAd3aCM");

#[program]
pub mod liq {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
