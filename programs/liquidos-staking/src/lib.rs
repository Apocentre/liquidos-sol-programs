#![allow(unexpected_cfgs)]
pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;
pub mod staking;

use anchor_lang::prelude::*;
use crate::instructions::{
  initialize::*, update_state::*, create_pool::*, deposit::*, withdraw::*,
  read_pending_reward::*, init_user_info::*,
};

declare_id!("HxbEKU2EdXw5xK5XnG8XWK2uMNCj1A69N3YkD65CcRcb");

#[program]
pub mod liquidos_staking {
  use super::*;

  /// Initialize
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `liquidos_curve_program` - The liquidos-curve program
  /// * `liquidos_curve_state` - The state of the main liquidos curve program
  /// * `treasury` - The treasury that will collect protocol fees
  /// * `staking_duration` - The total duration of each staking pool i.e. for how long users can stake and earn rewards.
  /// * `staking_delay` - How much the staking will be delayed for (in secs) from the moment the pool is created
  /// * `withdraw_delay` - Defines the seconds from the start of the pool that users will be able to withdraw their stake
  /// * `protocol_fee` - The fee in the reward token the protocol receives.
  pub fn initialize(
    ctx: Context<Initialize>,
    liquidos_curve_program: Pubkey,
    liquidos_curve_state: Pubkey,
    treasury: Pubkey,
    staking_duration: i64,
    staking_delay: i64,
    withdraw_delay: i64,
    protocol_fee: u16,
  ) -> Result<()> {
    processors::initialize::exec(
      ctx,
      liquidos_curve_program,
      liquidos_curve_state,
      treasury,
      staking_duration,
      staking_delay,
      withdraw_delay,
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
  /// * `staking_duration` - The total duration of each staking pool i.e. for how long users can stake and earn rewards.
  /// * `staking_delay` - How much the staking will be delayed for  (in secs) from the moment the pool is created
  /// * `withdraw_delay` - Defines the seconds from the start of the pool that users will be able to withdraw their stake
  /// * `protocol_fee` - The fee in the reward token the protocol receives.
  pub fn update_state(
    ctx: Context<UpdateState>,
    staking_duration: i64,
    staking_delay: i64,
    withdraw_delay: i64,
    protocol_fee: u16,
  ) -> Result<()> {
    processors::update_state::exec(
      ctx,
      staking_duration,
      staking_delay,
      withdraw_delay,
      protocol_fee,
    )
  }

  /// CreatePool
  /// 
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `total_rewards` - Total amount of rewards to be distributed. Note the caller (liquidos program) of this ix should
  ///                      first transfer this amount to `reward_token_vault_ata`
  pub fn create_pool(ctx: Context<CreatePool>, total_rewards: u64) -> Result<()> {
    processors::create_pool::exec(ctx, total_rewards)
  }

  /// InitUserInfo
  /// 
  /// Creates several user accounts. We separate that from deposit and withdraw due to stack limitation issues
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  pub fn init_user_info(ctx: Context<InitUserInfo>) -> Result<()> {
    processors::init_user_info::exec(ctx)
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

  /// View function that returns the pending rewards for the given account
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `user` - The account pending rewards will be calculated for
  /// * `state` - The state of this instance of program
  /// * `reward_token` - The reward token of the pool
  /// * `staking_token` - The staking token
  pub fn read_pending_reward(
    ctx: Context<ReadPendingReward>,
    _user: Pubkey,
    _state: Pubkey,
    _reward_token: Pubkey,
    _staking_token: Pubkey,
  ) -> Result<u64> {
    processors::read_pending_reward::exec(ctx)
  }
}
