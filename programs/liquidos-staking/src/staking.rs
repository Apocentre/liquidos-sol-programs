use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use crate::{
  account_data::pool_info::PoolInfo, program_error::ErrorCode,
};

pub fn harvest(pool_info: &mut PoolInfo) -> Result<()> {
  let now = Clock::get().unwrap().unix_timestamp;

  let last_harvest_ts = pool_info.last_harvest_ts;
  let round_end_ts = pool_info.round_end_ts;

  require!(now != last_harvest_ts, ErrorCode::SameBlockHarvest);
  require!(last_harvest_ts != round_end_ts, ErrorCode::RoundEnd);

  if now < round_end_ts {
    let acc_reward = ((now - last_harvest_ts) as u64).safe_mul(pool_info.reward_per_sec)?;
    pool_info.pending_reward = pool_info.pending_reward.safe_add(acc_reward)?;
    pool_info.last_harvest_ts = now;
  } else {
    let acc_reward = ((round_end_ts - last_harvest_ts) as u64).safe_mul(pool_info.reward_per_sec)?;
    pool_info.pending_reward = pool_info.pending_reward.safe_add(acc_reward)?;
    pool_info.last_harvest_ts = round_end_ts;
  }

  todo!()
}
