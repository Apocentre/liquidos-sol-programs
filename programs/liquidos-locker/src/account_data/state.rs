use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct State {
  /// The owner that can handle various admin related tasks
  pub owner: Pubkey,
  pub escrow_bump: u8,
}

impl State {
  pub const MAX_SIZE: usize = 8 + Self::INIT_SPACE;

  pub fn new(owner: Pubkey, escrow_bump: u8) -> Self {
    Self {owner, escrow_bump}
  }
}
