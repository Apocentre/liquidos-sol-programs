use anchor_lang::prelude::*;
use crate::{account_data::state::State, instructions::initialize::Initialize};

pub fn exec(
  ctx: Context<Initialize>,
  onlybags_program: Pubkey,
  onlybags_state: Pubkey,
  treasury: Pubkey,
  staking_duration: i64,
  protocol_fee: u16,
) -> Result<()> {
  let owner = ctx.accounts.owner.key();
  let state = &mut ctx.accounts.state;
  **state = State::new(
    owner,
    onlybags_program,
    onlybags_state,
    treasury,
    staking_duration,
    protocol_fee,
    ctx.bumps.pool_authority,
  );

  Ok(())
}
