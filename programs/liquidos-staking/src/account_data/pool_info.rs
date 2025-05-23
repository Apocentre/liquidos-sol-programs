use std::mem::size_of;
use anchor_lang::prelude::*;

#[account]
pub struct PoolInfo {
  /// Rewards earned per seconds
  pub reward_per_sec: u64,
  /// Accumulated reward per share
  pub acc_reward_per_share: u64,
  /// Total amount of rewards in reward_token
  pub total_claimed: u64,
  /// Total amount of BAGS currently staked in this pool
  pub total_staked: u64,
  /// The staking token of this pool. This is gonna be the native Liquidos token
  pub staking_token: Pubkey,
}

impl PoolInfo {
  pub const MAX_SIZE: usize = 8 + size_of::<Self>();

  pub fn new(staking_token: Pubkey) -> Self {
    Self {
      reward_per_sec: 0,
      acc_reward_per_share: 0,
      total_claimed: 0,
      total_staked: 0,
      staking_token,
    }
  }
}
