use anchor_lang::prelude::*;
use crate::instructions::update_state::UpdateState;

pub fn exec(
  ctx: Context<UpdateState>,
  staking_duration: i64,
  staking_delay: i64,
  protocol_fee: u16,
) -> Result<()> {
  let state = &mut ctx.accounts.state.load_mut()?;
  state.staking_duration = staking_duration;
  state.staking_delay = staking_delay;
  state.protocol_fee = protocol_fee;

  Ok(())
}
