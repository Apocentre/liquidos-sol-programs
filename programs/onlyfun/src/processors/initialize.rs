use anchor_lang::prelude::*;
use crate::{account_data::state::State, instructions::initialize::Initialize};

pub fn exec(
  ctx: Context<Initialize>,
  operators: Vec<Pubkey>,
  current_sol_target: u64,
) -> Result<()> {
  let owner = ctx.accounts.owner.key();
  let state = &mut ctx.accounts.state;
  **state = State::new(owner, operators, current_sol_target, ctx.bumps.cpi_authority);

  Ok(())
}
