use anchor_lang::prelude::*;
use crate::{account_data::state::State, instructions::initialize::Initialize};

pub fn exec(
  ctx: Context<Initialize>,
  treasury: Pubkey,
  protocol_fee: u64,
) -> Result<()> {
  let owner = ctx.accounts.owner.key();
  let state = &mut ctx.accounts.state;
  **state = State::new(
    owner,
    treasury,
    protocol_fee,
  );

  Ok(())
}
