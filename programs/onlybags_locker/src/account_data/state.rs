
use std::mem::size_of;
use anchor_lang::prelude::*;

#[account]
pub struct State {
  /// The owner that can handle various admin related tasks
  pub owner: Pubkey,
}

impl State {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();

  pub fn new(owner: Pubkey) -> Self {
    Self {owner}
  }
}
