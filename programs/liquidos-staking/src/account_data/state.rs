use std::mem::size_of;
use anchor_lang::prelude::*;

#[account(zero_copy)]
pub struct State {
  /// The owner that can handle various admin related tasks
  pub owner: Pubkey,
  /// The Liquidos curve program
  pub liquidos_curve_program: Pubkey,
  /// The state of the main Liquidos curve program
  pub liquidos_curve_state: Pubkey,
  /// The treasury that will collect protocol fees
  pub treasury: Pubkey,
  /// The total duration of each staking pool (in secs) i.e. for how long users can stake and earn rewards.
  /// This value also decides what the rewards per second will be
  pub staking_duration: i64,
  /// How long the staking will be delayed for (in secs) from the moment the pool is created
  pub staking_delay: i64,
  /// How long reward claims willbe delayed for from the start of the pool. i.e. seconds from the start of the pool
  /// that stakers can start claiming rewards
  /// DEPRECATED in v2. We keep it for backward compatibility so the share of the account data doesn't change
  pub claim_delay: i64,
  /// Defines the seconds from the start of the pool that users will be able to withdraw their stake
  pub withdraw_delay: i64,
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
    liquidos_curve_program: Pubkey,
    liquidos_curve_state: Pubkey,
    treasury: Pubkey,
    staking_duration: i64,
    staking_delay: i64,
    withdraw_delay: i64,
    protocol_fee: u16,
    pool_authority_bump: u8,
  ) -> Self {
    Self {
      owner,
      liquidos_curve_program,
      liquidos_curve_state,
      treasury,
      staking_duration,
      staking_delay,
      claim_delay: i64::MAX,
      withdraw_delay,
      pool_count: 0,
      protocol_fee,
      pool_authority_bump,
      _padding: [0; 3]
    }
  }
}
