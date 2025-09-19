use anchor_lang::prelude::*;
use crate::instructions::update_state::UpdateState;

pub fn exec(
  ctx: Context<UpdateState>,
  trade_fee_bps: u64,
) -> Result<()> {
  let state = &mut ctx.accounts.state;
  state.trade_fee_bps = trade_fee_bps;

  Ok(())
}
