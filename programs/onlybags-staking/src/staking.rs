use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use crate::account_data::{pool_info::PoolInfo, state::State};

pub const NORMALIZATION_FACTOR: u64 = 1_000_000;

pub fn update_pool(pool_info: &mut PoolInfo) -> Result<()> {
  let now = Clock::get().unwrap().unix_timestamp;

  if now > pool_info.last_reward_ts {
    if pool_info.total_staked > 0 {
      pool_info.acc_reward_per_share = calc_acc_reward_per_share(pool_info, now)?;
    }

    pool_info.last_reward_ts = now;
  }

  Ok(())
}

fn calc_reward(pool_info: &PoolInfo, now: i64) -> Result<u64> {
  let time_elapsed = (now - pool_info.last_reward_ts) as u64;
  let reward = time_elapsed.safe_mul(pool_info.reward_per_sec)?;

  Ok(reward as u64)
}

fn calc_acc_reward_per_share(pool_info: &PoolInfo, now: i64) -> Result<u64> {
  let reward = calc_reward(pool_info, now)?;
  let acc_reward_per_share = reward
  .safe_mul(NORMALIZATION_FACTOR)?
  .safe_div(pool_info.total_staked as u64)?;

  let new_acc_reward_per_share = pool_info.acc_reward_per_share.safe_add(acc_reward_per_share)?;

  Ok(new_acc_reward_per_share)
}
