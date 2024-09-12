
use std::mem::size_of;
use anchor_lang::prelude::*;

pub const MAX_OPERATORS: usize = 5;

#[account]
pub struct State {
  /// The owner that can handle various admin related teasks
  pub owner: Pubkey,
  /// The treasury account that receives fees
  pub treasury: Pubkey,
  /// Current protocol fees i.e. fees collected on each swap
  pub protocol_fee_bps: u64,
}

impl State {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();

  pub fn new(
    owner: Pubkey,
    treasury: Pubkey,
    protocol_fee_bps: u64,
  ) -> Self {
    Self {
      owner,
      treasury,
      protocol_fee_bps,
    }
  }
}
