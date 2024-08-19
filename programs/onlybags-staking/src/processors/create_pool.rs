use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use crate::{
  account_data::pool_info::PoolInfo, instructions::create_pool::CreatePool
};

pub fn exec(ctx: Context<CreatePool>, total_rewards: u64) -> Result<()> {
  let pool_info = &mut ctx.accounts.pool_info;
  let state = &ctx.accounts.state;
  let now = Clock::get().unwrap().unix_timestamp;
  let reward_per_sec = (state.staking_duration as u64).safe_div(total_rewards)?;
  let end_ts = (now as u64).safe_add(state.staking_duration as u64)? as i64;

  **pool_info = PoolInfo::new(
    reward_per_sec,
    now,
    end_ts,
    total_rewards,
    ctx.accounts.reward_token.key(),
    ctx.accounts.state.protocol_fee,
  );

  let state = &mut ctx.accounts.state;
  state.pool_count += 1;

  Ok(())
}
