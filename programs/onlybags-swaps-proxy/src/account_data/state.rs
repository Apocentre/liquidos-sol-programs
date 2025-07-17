use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct State {
  /// The owner that can handle various admin related tasks
  pub owner: Pubkey,
  /// The treasury account that receives fees
  pub treasury: Pubkey,
  /// Current protocol fees i.e. fees collected on each swap
  pub protocol_fee_bps: u64,
}

impl State {
  pub const MAX_SIZE: usize = 8 + Self::INIT_SPACE;

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
