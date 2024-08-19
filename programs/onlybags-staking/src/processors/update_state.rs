use anchor_lang::prelude::*;
use crate::instructions::update_state::UpdateState;

pub fn exec(
  ctx: Context<UpdateState>,
  staking_duration: i64,
  staking_token: Pubkey,
  protocol_fee: u16,
) -> Result<()> {
  let state = &mut ctx.accounts.state;
  state.staking_duration = staking_duration;
  state.staking_token = Some(staking_token);
  state.protocol_fee = protocol_fee;

  Ok(())
}
