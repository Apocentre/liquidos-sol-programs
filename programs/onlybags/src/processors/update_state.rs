use anchor_lang::prelude::*;
use crate::instructions::update_state::UpdateState;

pub fn exec(
  ctx: Context<UpdateState>,
  staking_program_state: Pubkey,
  protocol_fee: u64,
  trade_fee_bps: u64,
  creator_fee: u64,
  total_token_supply: u64,
  staking_allocation: u64,
) -> Result<()> {
  let state = &mut ctx.accounts.state;
  state.staking_program_state = Some(staking_program_state);
  state.protocol_fee = protocol_fee;
  state.trade_fee_bps = trade_fee_bps;
  state.creator_fee = creator_fee;
  state.total_token_supply = total_token_supply;
  state.staking_allocation = staking_allocation;

  Ok(())
}
