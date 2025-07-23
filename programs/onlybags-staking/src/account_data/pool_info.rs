use std::mem::size_of;
use anchor_lang::prelude::*;

#[account(zero_copy)]
pub struct PoolInfo {
  /// Accumulated reward per share
  pub acc_reward_per_share: u64,
  /// The last time reward was calculated
  pub last_reward_ts: i64,
  /// The ts when the pools opens for staking
  pub start_ts: i64,
  /// The timestamp when the pool is closed so no more deposits are allowed
  pub end_ts: i64,
  /// The staking period duratio
  pub staking_duration: i64,
  /// The ts of the first stake. We store this value to make sure that all rewards are distributed
  pub first_stake_ts: i64,
  /// The ts when user can claim rewards
  /// DEPRECATED in v2. We keep it for backward compatibility so the share of the account data doesn't change
  pub timelock_ts: i64,
  /// The ts when user can withdraw his stake
  pub withdraw_lock_ts: i64,
  /// Total amount of rewards in reward_token
  pub total_reward: u64,
  /// Total amount of rewards in reward_token
  pub total_claimed: u64,
  /// Total amount of BAGS currently staked in this pool
  pub total_staked: u64,
  /// Rewards earned per seconds
  pub reward_per_sec: u64,
  /// The staking token of this pool
  pub staking_token: Pubkey,
  /// The reward token of this pool
  pub reward_token: Pubkey,
  /// The fee in the reward token the protocol receives
  pub protocol_fee: u16,
  // https://github.com/coral-xyz/anchor/issues/2759#issuecomment-1874845771
  _padding: [u8; 6]
}

impl PoolInfo {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();

  pub fn new(
    reward_per_sec: u64,
    last_reward_ts: i64,
    start_ts: i64,
    staking_duration: i64,
    withdraw_lock_ts: i64,
    total_reward: u64,
    staking_token: Pubkey,
    reward_token: Pubkey,
    protocol_fee: u16,
  ) -> Self {
    Self {
      acc_reward_per_share: 0,
      last_reward_ts,
      start_ts,
      end_ts: 0,
      staking_duration,
      first_stake_ts: 0,
      timelock_ts: i64::MAX,
      withdraw_lock_ts,
      total_staked: 0,
      total_reward,
      total_claimed: 0,
      staking_token,
      reward_token,
      reward_per_sec,
      protocol_fee,
      _padding: [0; 6]
    }
  }
}
