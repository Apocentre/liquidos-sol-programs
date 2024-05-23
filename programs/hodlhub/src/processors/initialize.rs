use anchor_lang::prelude::*;
use crate::{account_data::state::State, instructions::initialize::Initialize};

pub fn exec(
  ctx: Context<Initialize>,
  treasury: Pubkey,
  sol_target: u64,
  protocol_fee_bps: u64,
  trade_fee_bps: u64,
) -> Result<()> {
  let owner = ctx.accounts.owner.key();
  let state = &mut ctx.accounts.state;
  **state = State::new(
    owner,
    treasury,
    sol_target,
    protocol_fee_bps,
    trade_fee_bps,
    ctx.bumps.cpi_authority,
  );

  Ok(())
}
