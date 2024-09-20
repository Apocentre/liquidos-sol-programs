
use std::mem::size_of;
use anchor_lang::prelude::*;

#[account]
pub struct TokenLock {
  /// Total tokens locked
  pub total_locked: u64,
  /// Flag that determined if account is created
  pub initialized: bool,
  pub bump: u8
}

impl TokenLock {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();

  pub fn new(bump: u8) -> Self {
    Self {
      total_locked: 0,
      initialized: true,
      bump,
    }
  }
}
