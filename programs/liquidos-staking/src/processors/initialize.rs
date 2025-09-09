use anchor_lang::prelude::*;
use crate::{account_data::state::State, instructions::initialize::Initialize};

pub fn exec(
  ctx: Context<Initialize>,
  liquidos_curve_program: Pubkey,
  liquidos_curve_state: Pubkey,
  treasury: Pubkey,
  staking_duration: i64,
  staking_delay: i64,
  withdraw_delay: i64,
  protocol_fee: u16,
) -> Result<()> {
  let owner = ctx.accounts.owner.key();
  // We need to call load_init only once so anchor adds the discriminator.
  let state = &mut ctx.accounts.state.load_init()?;

  **state = State::new(
    owner,
    liquidos_curve_program,
    liquidos_curve_state,
    treasury,
    staking_duration,
    staking_delay,
    withdraw_delay,
    protocol_fee,
    ctx.bumps.pool_authority,
  );

  Ok(())
}
