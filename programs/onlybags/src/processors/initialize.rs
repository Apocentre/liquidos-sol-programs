use anchor_lang::prelude::*;
use crate::{account_data::state::State, instructions::initialize::Initialize};

pub fn exec(
  ctx: Context<Initialize>,
  treasury: Pubkey,
  protocol_fee: u64,
  trade_fee_bps: u64,
  creator_fee: u64,
  total_token_supply: u64,
  staking_allocation_bps: u64,
) -> Result<()> {
  let owner = ctx.accounts.owner.key();
  let state = &mut ctx.accounts.state;
  **state = State::new(
    owner,
    treasury,
    protocol_fee,
    trade_fee_bps,
    creator_fee,
    total_token_supply,
    staking_allocation_bps,
  );

  Ok(())
}
