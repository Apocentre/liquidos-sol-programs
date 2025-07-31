#![allow(unexpected_cfgs)]
pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;

use anchor_lang::prelude::*;
use crate::instructions::{initialize::*, lock::*, unlock::*};

declare_id!("7Sntio7WMpXkPWTQMXLVYoH7hqwfLn1i4JP7xUFaDkaK");

#[program]
pub mod liquidos_locker {
  use super::*;

  /// Initialize
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    processors::initialize::exec(ctx)
  }

  /// Lock
  /// Allow anyone to lock the provided amount of tokens for the given duration
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `amount` - The amount to lock
  /// * `duration` - The duration of the lock
  pub fn lock(ctx: Context<Lock>, amount: u64, duration: i64, _test_ts: i64) -> Result<()> {
    processors::lock::exec(ctx, amount, duration, _test_ts)
  }

  /// Lock
  /// Allow anyone to unlock full amount after lock expiration
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  pub fn unlock(ctx: Context<UnLock>, _test_ts: i64) -> Result<()> {
    processors::unlock::exec(ctx, _test_ts)
  }
}
