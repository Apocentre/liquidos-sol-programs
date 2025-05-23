use anchor_lang::prelude::*;

use crate::constants::SPACE_MARGIN;

#[account]
#[derive(InitSpace)]
pub struct PoolInfo {
  /// Rewards earned per seconds
  pub reward_per_sec: u64,
  /// Accumulated reward per share
  pub acc_reward_per_share: u64,
  /// Total amount of rewards in reward_token
  pub total_claimed: u64,
  /// Total amount of BAGS currently staked in this pool
  pub total_staked: u64,
  /// This value stores the pending rewards when a new distribution of funds is done. This
  /// way we guarantee that all rewards are distributed to stakers even when staking params
  /// due to new funds distribution are changed.
  pub pending_reward: u64,
  /// The timestamp of the end of the current distribution round
  pub round_end_ts: i64,
  /// The rewards distribution round duration e.g. 1 day in seconds
  pub round_duration_secs: i64,
  /// The last time reward was calculated
  pub last_harvest_ts: i64,
  /// The staking token of this pool. This is gonna be the native Liquidos token
  pub staking_token: Pubkey,
  pub bump: u8,
}

impl PoolInfo {
  pub const MAX_SIZE: usize = 8 + Self::INIT_SPACE + SPACE_MARGIN;

  pub fn new(staking_token: Pubkey, round_duration_secs: i64, bump: u8) -> Self {
    Self {
      reward_per_sec: 0,
      acc_reward_per_share: 0,
      total_claimed: 0,
      total_staked: 0,
      pending_reward: 0,
      round_end_ts: 0,
      round_duration_secs,
      last_harvest_ts: 0,
      staking_token,
      bump,
    }
  }
}
