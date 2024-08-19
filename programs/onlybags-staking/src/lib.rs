pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;
pub mod staking;

use anchor_lang::prelude::*;
use crate::instructions::{
  initialize::*, update_state::*, create_pool::*, deposit::*, withdraw::*,
};

declare_id!("BysFb46aUfoNS9BEuAA63Ut61qSz4gjiJgNExN8KtYem");

#[program]
pub mod onlybags_staking {
  use super::*;

  /// Initialize
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `onlybags_state` - The state of the main Onlybags program
  /// * `treasury` - The treasury that will collect protocol fees
  /// * `staking_duration` - The total duration of each staking pool i.e. for how long users can stake and earn rewards.
  /// * `protocol_fee` - The fee in the reward token the protocol receives.
  
  pub fn initialize(
    ctx: Context<Initialize>,
    onlybags_state: Pubkey,
    treasury: Pubkey,
    staking_duration: i64,
    protocol_fee: u16,
  ) -> Result<()> {
    processors::initialize::exec(
      ctx,
      onlybags_state,
      treasury,
      staking_duration,
      protocol_fee,
    )
  }

  /// UpdateState
  ///
  /// Allows admit to update the state
  /// 
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `staking_token` - The mint account of the staking token. This will be Onlybag's token ($BAGS)
  /// * `staking_duration` - The total duration of each staking pool i.e. for how long users can stake and earn rewards.
  /// * `protocol_fee` - The fee in the reward token the protocol receives.
  pub fn update_state(
    ctx: Context<UpdateState>,
    staking_token: Pubkey,
    staking_duration: i64,
    protocol_fee: u16,
  ) -> Result<()> {
    processors::update_state::exec(ctx, staking_token, staking_duration, protocol_fee)
  }

  /// CreatePool
  ///
  /// Allows admit to update the state
  /// 
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `total_rewards` - Total amount of rewards to be distributed. Note the caller (Onlybags program) of this ix should
  ///                      first transfer this amount to `reward_token_vault_ata`
  pub fn create_pool(ctx: Context<CreatePool>, total_rewards: u64) -> Result<()> {
    processors::create_pool::exec(ctx, total_rewards)
  }

  /// Deposit
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `total_rewards` - The amount to deposit
  pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    processors::deposit::exec(ctx, amount)
  }

  /// Withdraw
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `total_rewards` - The amount to withdraw
  pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    processors::withdraw::exec(ctx, amount)
  }  
}
