pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;
pub mod raydium;

use anchor_lang::prelude::*;
use crate::instructions::initialize::*;

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
    protocol_fee: u64,
  ) -> Result<()> {
    processors::initialize::exec(
      ctx,
      treasury,
      protocol_fee,
    )
  }
}
