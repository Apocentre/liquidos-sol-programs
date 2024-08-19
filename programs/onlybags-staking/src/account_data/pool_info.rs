use std::mem::size_of;

use anchor_lang::prelude::*;

#[account]
pub struct PoolInfo {
  /// Accumulated reward per share
  pub acc_reward_per_share: u128,
  /// The last time reward was calculated
  pub last_reward_ts: i64,
  /// Total amount of BAGS currently staked in this pool
  pub total_staked: u64,
  /// The reward token of this pool
  pub reward_token: Pubkey,
  /// The fee in the reward token the protocol receives
  pub protocol_fee: u16,
}

impl PoolInfo {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();

  pub fn new(
    last_reward_ts: i64,
    reward_token: Pubkey,
    protocol_fee: u16,
  ) -> Self {
    Self {
      acc_reward_per_share: 0,
      last_reward_ts,
      total_staked: 0,
      reward_token,
      protocol_fee,
    }
  }
}
