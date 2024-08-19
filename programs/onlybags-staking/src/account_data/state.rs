use std::mem::size_of;
use anchor_lang::prelude::*;

#[account]
pub struct State {
  /// The owner that can handle various admin related teasks
  pub owner: Pubkey,
  /// The total duration of each staking pool i.e. for how long users can stake and earn rewards.
  /// This value also decides what the rewards per second will be
  pub staking_duration: i64,
  /// The mint account of the staking token. This will be Onlybag's token ($BAGS)
  pub staking_token: Option<Pubkey>,
  /// Total number of pools created
  pub pool_count: u16,
  /// The fee in the reward token the protocol receives
  pub protocol_fee: u16,
}

impl State {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();

  pub fn new(owner: Pubkey, staking_duration: i64, protocol_fee: u16,) -> Self {
    Self {
      owner,
      staking_duration,
      staking_token: None,
      pool_count: 0,
      protocol_fee,
    }
  }
}
