use anchor_lang::prelude::*;
use crate::{instructions::read_pending_reward::ReadPendingReward, staking::get_pending_rewards};

pub fn exec(ctx: Context<ReadPendingReward>) -> Result<u64> {
  get_pending_rewards(
    &ctx.accounts.pool_info,
    &ctx.accounts.user_info,
  )
}
