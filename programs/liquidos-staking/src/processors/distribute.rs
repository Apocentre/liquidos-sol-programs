use anchor_lang::prelude::*;
use crate::{instructions::distribute::Distribute, staking::harvest};

pub fn exec(ctx: Context<Distribute>, amount: u64) -> Result<()> {
  harvest();
  Ok(())
}
