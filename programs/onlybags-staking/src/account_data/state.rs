use anchor_lang::prelude::*;

#[account]
pub struct State {
  /// The owner that can handle various admin related teasks
  pub owner: Pubkey,
  /// Total number of pools created
  pub pool_count: u16,
  /// The total duration of each staking pool i.e. for how long users can stake and earn rewards.
  /// This value also decides what the rewards per second will be
  pub staking_duration: u64,
  /// The mint account of the reward token. This will be Onlybag's token ($BAGS)
  pub reward_token: Pubkey,
}
