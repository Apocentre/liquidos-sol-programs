use anchor_lang::prelude::*;
use crate::{account_data::state::State, instructions::initialize::Initialize};

pub fn exec(
  ctx: Context<Initialize>,
  staking_duration: i64,
  reward_token: Option<Pubkey>,
) -> Result<()> {
  let owner = ctx.accounts.owner.key();
  let state = &mut ctx.accounts.state;
  **state = State::new(
    owner,
    staking_duration,
    reward_token,
  );

  Ok(())
}
