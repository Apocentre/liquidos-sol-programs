use std::mem::size_of;
use anchor_lang::prelude::*;
use bytemuck::Zeroable;

#[account(zero_copy)]
pub struct State {
  /// The owner that can handle various admin related teasks
  pub owner: Pubkey,
  /// The treasury that will collect protocol fees
  pub treasury: Pubkey,
  /// The state of the main Onlybags program
  pub onlybags_state: Pubkey,
  /// The total duration of each staking pool i.e. for how long users can stake and earn rewards.
  /// This value also decides what the rewards per second will be
  pub staking_duration: i64,
  /// The mint account of the staking token. This will be Onlybag's token ($BAGS)
  pub staking_token: Pubkey,
  /// Total number of pools created
  pub pool_count: u16,
  /// The fee in the reward token the protocol receives
  pub protocol_fee: u16,
  /// The PDA bump of this account
  pub pool_authority_bump: u8,
  // https://github.com/coral-xyz/anchor/issues/2759#issuecomment-1874845771
  _padding: [u8; 3]
}

impl State {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();

  pub fn new(
    owner: Pubkey,
    treasury: Pubkey,
    onlybags_state: Pubkey,
    staking_duration: i64,
    protocol_fee: u16,
    pool_authority_bump: u8,
  ) -> Self {
    Self {
      owner,
      treasury,
      onlybags_state,
      staking_duration,
      staking_token: Pubkey::zeroed(),
      pool_count: 0,
      protocol_fee,
      pool_authority_bump,
      _padding: [0; 3]
    }
  }
}
