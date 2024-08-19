pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;

use anchor_lang::prelude::*;
use crate::instructions::initialize::*;

declare_id!("8c3Znxt8mLm3kbmJBYkbKJSsEq7SCxDntNgRJeeGbr8W");

#[program]
pub mod onlybags_staking {
  use super::*;

  /// Initialize
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `onlybags_state` - The state of the main Onlybags program
  /// * `staking_duration` - The total duration of each staking pool i.e. for how long users can stake and earn rewards.
  /// * `protocol_fee` - The fee in the reward token the protocol receives.
  
  pub fn initialize(
    ctx: Context<Initialize>,
    onlybags_state: Pubkey,
    staking_duration: i64,
    protocol_fee: u16,
  ) -> Result<()> {
    processors::initialize::exec(
      ctx,
      onlybags_state,
      staking_duration,
      protocol_fee,
    )
  }
}
