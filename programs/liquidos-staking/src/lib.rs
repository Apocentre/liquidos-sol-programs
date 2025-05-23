#![allow(unexpected_cfgs)]
pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;


use anchor_lang::prelude::*;

declare_id!("NBiqeP8VynsHfaUNP5dWru2T8ioBAmzurYxn7UmS7KJ");

#[program]
pub mod liquidos_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
