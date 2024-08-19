use std::mem::size_of;
use anchor_lang::prelude::*;

#[account]
pub struct State {
  /// The owner that can handle various admin related teasks
  pub owner: Pubkey,
  /// The state of the main Onlybags program
  pub onlybags_state: Pubkey,
  /// The total duration of each staking pool i.e. for how long users can stake and earn rewards.
  /// This value also decides what the rewards per second will be
  pub staking_duration: i64,
  /// The mint account of the staking token. This will be Onlybag's token ($BAGS)
  pub staking_token: Option<Pubkey>,
  /// Total number of pools created
  pub pool_count: u16,
  /// The fee in the reward token the protocol receives
  pub protocol_fee: u16,
  /// The PDA bump of this account
  pub pool_authority_bump: u8,
}

impl State {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();

  pub fn new(
    owner: Pubkey,
    onlybags_state: Pubkey,
    staking_duration: i64,
    protocol_fee: u16,
    pool_authority_bump: u8,
  ) -> Self {
    Self {
      owner,
      onlybags_state,
      staking_duration,
      staking_token: None,
      pool_count: 0,
      protocol_fee,
      pool_authority_bump,
    }
  }
}
