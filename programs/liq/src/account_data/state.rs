use anchor_lang::prelude::*;
use crate::constants::SPACE_MARGIN;

#[account]
#[derive(InitSpace)]
pub struct State {
  /// The owner that can handle various admin related tasks
  pub owner: Pubkey,
}

impl State {
  pub const MAX_SIZE: usize = 8 + Self::INIT_SPACE + SPACE_MARGIN;

  pub fn new(owner: Pubkey) -> Self {
    Self {
      owner,
    }
  }
}
