pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;

use anchor_lang::prelude::*;
use crate::instructions::initialize::*;

declare_id!("CmccctV39SQpEiVsK3hgRo6i6QW55pLBTSsEmDLw9AXY");

#[program]
pub mod onlybags_locker {
  use super::*;

  /// Initialize
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    processors::initialize::exec(ctx)
  }
}
