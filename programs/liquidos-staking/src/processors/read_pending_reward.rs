use anchor_lang::prelude::*;
use crate::{instructions::read_pending_reward::ReadPendingReward, staking::get_pending_rewards};

pub fn exec(ctx: Context<ReadPendingReward>) -> Result<u64> {
  let pool_info = ctx.accounts.pool_info.load()?;
  get_pending_rewards(
    &pool_info,
    &ctx.accounts.user_info,
  )
}
