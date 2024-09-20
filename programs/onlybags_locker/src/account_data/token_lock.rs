
use std::mem::size_of;
use anchor_lang::prelude::*;

#[account]
pub struct TokenLock {
  /// Total tokens locked
  pub total_locked: u64,
}

impl TokenLock {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();

  pub fn new() -> Self {
    Self {
      total_locked: 0,
    }
  }
}
