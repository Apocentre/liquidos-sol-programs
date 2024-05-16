use anchor_lang::prelude::*;

declare_id!("2d6f7qg9SnGaLSN1EejmD3da72bJppqmKnB6C21zFNHj");

#[program]
pub mod onlyfun {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
