pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;
pub mod raydium;

use anchor_lang::prelude::*;
use crate::instructions::{initialize::*, swap::*};

declare_id!("GdyU6f76XkkeWF63CqhXDVXqx56Ldva4saVxEDpWJiaY");

#[program]
pub mod onlybags_swap_proxy {
  use super::*;

  /// Initialize
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `treasury` - The treasury account that receives fees
  /// * `protocol_fee` - Current protocol fees i.e. fees collected on each swap
  pub fn initialize(
    ctx: Context<Initialize>,
    treasury: Pubkey,
    protocol_fee_bps: u64,
  ) -> Result<()> {
    processors::initialize::exec(
      ctx,
      treasury,
      protocol_fee_bps,
    )
  }

  /// SwapBaseInput
  /// 
  /// Use this when user enters the amount of base tokens he wants so sell
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `amount_in` - The amount of base token to sell
  /// * `minimum_amount_out` - minimum amount of the out token to receive
  pub fn swap_base_input(
    ctx: Context<Swap>,
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
  /// Use this when user enters the amount of non-base tokens he want to receive
  /// 
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `max_amount_in` - The max amount of base tokens to be sold
  /// * `amount_out_less_fee` - The amount of non-base tokens user wants to buy
  pub fn swap_base_output(
    ctx: Context<Swap>,
    max_amount_in: u64,
    amount_out_less_fee: u64,
  ) -> Result<()> {
    processors::swap_base_output::exec(
      ctx,
      max_amount_in,
      amount_out_less_fee,
    )
  }
}
