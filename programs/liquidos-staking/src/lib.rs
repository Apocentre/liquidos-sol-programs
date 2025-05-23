#![allow(unexpected_cfgs)]
pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;


use anchor_lang::prelude::*;
use instructions::initialize::*;

declare_id!("NBiqeP8VynsHfaUNP5dWru2T8ioBAmzurYxn7UmS7KJ");

#[program]
pub mod liquidos_staking {
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
