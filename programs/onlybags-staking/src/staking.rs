use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use crate::account_data::{pool_info::PoolInfo, state::State};

pub const NORMALIZATION_FACTOR: u128 = 1_000_000_000;

pub fn update_pool(
  state: &State,
  pool_info: &mut PoolInfo
) -> Result<()> {
  let now = Clock::get().unwrap().unix_timestamp;

  if now > pool_info.last_reward_ts {
    if pool_info.total_staked > 0 {
      pool_info.acc_reward_per_share = calc_acc_reward_per_share(state, pool_info, now)?;
    }

    pool_info.last_reward_ts = now;
  }

  Ok(())
}

fn calc_reward(
  state: &State,
  pool_info: &PoolInfo,
  now: i64,
) -> StdResult<u128, ProgramError> {
  let time_elapsed = (now - pool_info.last_reward_ts) as u64;
  let reward = time_elapsed
    .safe_mul(state.reward_per_sec)?
    .safe_mul(pool_info.alloc_points as u64)?
    .safe_div(state.total_alloc_points as u64)?;

  Ok(reward as u128)
}

fn calc_acc_reward_per_share(
  state: &State,
  pool_info: &PoolInfo,
  now: i64,
) -> Result<u128, ProgramError> {
  let reward = calc_reward(state, pool_info, now)?;
  let acc_reward_per_share = reward
    .safe_mul(NORMALIZATION_FACTOR)?
    .safe_div(pool_info.total_staked as u128)?;

  let new_acc_reward_per_share = pool_info.acc_reward_per_share
    .safe_add(acc_reward_per_share)?;

  Ok(new_acc_reward_per_share)
}
