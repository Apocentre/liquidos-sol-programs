use anchor_lang::prelude::*;
use crate::{account_data::pool_info::PoolInfo, instructions::create_pool::CreatePool};

pub fn exec(ctx: Context<CreatePool>) -> Result<()> {
  let pool_info = &mut ctx.accounts.pool_info;

  **pool_info = PoolInfo::new(
    Clock::get().unwrap().unix_timestamp,
    ctx.accounts.reward_token.key(),
    ctx.accounts.state.protocol_fee,
  );

  let state = &mut ctx.accounts.state;
  state.pool_count += 1;

  Ok(())
}
