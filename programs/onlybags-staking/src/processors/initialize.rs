use anchor_lang::prelude::*;
use crate::{account_data::state::State, instructions::initialize::Initialize};

pub fn exec(
  ctx: Context<Initialize>,
  onlybags_program: Pubkey,
  onlybags_state: Pubkey,
  treasury: Pubkey,
  staking_duration: i64,
  staking_delay: i64,
  claim_delay: i64,
  protocol_fee: u16,
) -> Result<()> {
  let owner = ctx.accounts.owner.key();
  // We need to call load_init only once so anchor adds the discriminator.
  let state = &mut ctx.accounts.state.load_init()?;

  **state = State::new(
    owner,
    onlybags_program,
    onlybags_state,
    treasury,
    staking_duration,
    staking_delay,
    claim_delay,
    protocol_fee,
    ctx.bumps.pool_authority,
  );

  Ok(())
}
