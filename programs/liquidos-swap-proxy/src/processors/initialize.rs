use anchor_lang::prelude::*;
use crate::{account_data::state::{State, Treasury}, instructions::initialize::Initialize};

pub fn exec(
  ctx: Context<Initialize>,
  protocol_fee_bps: u64,
  treasuries: Vec<Treasury>,
) -> Result<()> {
  let owner = ctx.accounts.owner.key();
  let state = &mut ctx.accounts.state;
  **state = State::new(
    owner,
    protocol_fee_bps,
    treasuries,
  );

  Ok(())
}
