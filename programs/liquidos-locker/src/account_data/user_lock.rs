
use std::mem::size_of;
use anchor_lang::prelude::*;

#[account]
pub struct UserLock {
  /// Total tokens locked
  pub total_locked: u64,
  /// Starting date of the lock
  pub start_ts: i64,
  /// The duration of the lock
  pub duration: i64,
  /// Flag that determined if account is created
  pub initialized: bool,
  pub bump: u8,
}

impl UserLock {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();

  pub fn new(start_ts: i64, duration: i64, bump: u8) -> Self {
    Self {
      total_locked: 0,
      start_ts,
      duration,
      initialized: true,
      bump,
    }
  }
}
