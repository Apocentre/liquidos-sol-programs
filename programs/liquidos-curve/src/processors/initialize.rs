use anchor_lang::prelude::*;
use crate::{account_data::state::{State, Treasury}, instructions::initialize::Initialize};

pub fn exec(
  ctx: Context<Initialize>,
  treasuries: Vec<Treasury>,
  protocol_fee: u64,
  trade_fee_bps: u64,
  creator_fee: u64,
  total_token_supply: u64,
  staking_allocation: u64,
) -> Result<()> {
  let owner = ctx.accounts.owner.key();
  let state = &mut ctx.accounts.state;
  **state = State::new(
    owner,
    treasuries,
    protocol_fee,
    trade_fee_bps,
    creator_fee,
    total_token_supply,
    staking_allocation,
  )?;

  Ok(())
}
