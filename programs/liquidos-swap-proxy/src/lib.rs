#![allow(unexpected_cfgs)]
pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;
pub mod raydium;

use anchor_lang::prelude::*;
use crate::{
  instructions::{initialize::*, swap::*, update_state::*}, account_data::state::Treasury,
};

#[cfg(feature = "devnet")]
declare_id!("GdyU6f76XkkeWF63CqhXDVXqx56Ldva4saVxEDpWJiaY");

#[cfg(not(feature = "devnet"))]
declare_id!("5r8CvW57rPwHJJcyBhALpnJkpK5hMGabfrWpp83nMn7S");

#[program]
pub mod liquidos_swap_proxy {
  use super::*;

  /// Initialize
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `trade_fee_bps` - Current protocol fees i.e. fees collected on each swap
  /// * `treasuries` - The treasury accounts that receives fees and the corresponding portion each received from the trade_fee
  pub fn initialize(
    ctx: Context<Initialize>,
    trade_fee_bps: u64,
    treasuries: Vec<Treasury>,
  ) -> Result<()> {
    processors::initialize::exec(
      ctx,
      trade_fee_bps,
      treasuries,
    )
  }

  /// SwapBaseInput
  /// 
  /// Use this when user enters the amount of input tokens he wants so sell
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `amount_in` - The amount of input token  to sell
  /// * `minimum_amount_out` - minimum amount of the output to receive
  pub fn swap_base_input<'info>(
    ctx: Context<'_, '_, '_, 'info, Swap<'info>>,
    amount_in: u64,
    minimum_amount_out: u64,
  ) -> Result<()> {
    processors::swap_base_input::exec(
      ctx,
      amount_in,
      minimum_amount_out,
    )
  }

  /// SwapBaseOutput
  ///
  /// Use this when user enters the amount of output tokens he wants to receive
  /// 
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `max_amount_in` - The max amount of input tokes to be sold
  /// * `amount_out_less_fee` - The amount of output token user wants to buy
  pub fn swap_base_output<'info>(
    ctx: Context<'_, '_, '_, 'info, Swap<'info>>,
    max_amount_in: u64,
    amount_out_less_fee: u64,
  ) -> Result<()> {
    processors::swap_base_output::exec(
      ctx,
      max_amount_in,
      amount_out_less_fee,
    )
  }

  /// UpdateState
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `trade_fee_bps` - Current protocol fees i.e. fees collected on each swap
  pub fn update_state(
    ctx: Context<UpdateState>,
    trade_fee_bps: u64,
  ) -> Result<()> {
    processors::update_state::exec(ctx, trade_fee_bps)
  }
}
