#![allow(unexpected_cfgs)]
pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;
pub mod constants;
pub mod staking;


use anchor_lang::prelude::*;
use instructions::{initialize::*, distribute::*};

declare_id!("NBiqeP8VynsHfaUNP5dWru2T8ioBAmzurYxn7UmS7KJ");

#[program]
pub mod liquidos_staking {
  use super::*;

  /// Initialize
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `round_duration_secs` - The rewards distribution round duration e.g. 1 day in seconds
  pub fn initialize(ctx: Context<Initialize>, round_duration_secs: i64) -> Result<()> {
    processors::initialize::exec(ctx, round_duration_secs)
  }

  /// Distribute
  ///
  /// Allows a distributor to distribute funds. Basically, a distributor will send SOL to the 
  /// `pool_info` PDA which will then be distibuted amongt the stakers
  /// 
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `amount` - Amount to be distributed
  pub fn distribute(ctx: Context<Distribute>, amount: u64, _test_ts: i64) -> Result<()> {
    processors::distribute::exec(ctx, amount, _test_ts)
  }
}
