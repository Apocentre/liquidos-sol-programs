pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;
pub mod math;
pub mod raydium;

use anchor_lang::prelude::*;
use crate::instructions::{
  initialize::*, create_token::*, buy::*, sell::*,
};

declare_id!("2d6f7qg9SnGaLSN1EejmD3da72bJppqmKnB6C21zFNHj");

#[program]
pub mod hodlhub {
  use super::*;

  /// Initialize
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `operators` - The list of all operators that can run admin related tasks
  /// * `sol_target` - Current target of SOL each pool should receive before it goes to the 
  /// * `protocol_fee_bps` - te protocol fees (BPS)
  pub fn initialize(
    ctx: Context<Initialize>,
    operators: Vec<Pubkey>,
    sol_target: u64,
    protocol_fee_bps: u16,
  ) -> Result<()> {
    processors::initialize::exec(
      ctx,
      operators,
      sol_target,
      protocol_fee_bps,
    )
  }

  /// CreateToken
  ///
  /// Creates a new token
  /// 
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `name` - The name of the token (used in the metadata account)
  /// * `symbol` - The symbol of the token (used in the metadata account)
  /// * `uri` - The uri of the token (used in the metadata account)
  pub fn create_token(
    ctx: Context<CreateToken>,
    name: String,
    symbol: String,
    uri: String,
  ) -> Result<()> {
    processors::create_token::exec(ctx, name, symbol, uri)
  }

  /// Buy
  ///
  /// Buy tokens
  /// 
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `amount` - Amount of SOL buyer sells
  /// * `min_amount_out` - Min amount of tokens expected to receive (slippage protection)
  pub fn buy(
    ctx: Context<Buy>,
    amount: u64,
    min_amount_out: u64,
  ) -> Result<()> {
    processors::buy::exec(ctx, amount, min_amount_out)
  }

  /// Sell
  ///
  /// Sell tokens
  /// 
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `token_amount` - Amount of tokens seller sells
  /// * `min_sol_amount_out` - Min amount of SOL expected to receive (slippage protection)
  pub fn sell(
    ctx: Context<Sell>,
    token_amount: u64,
    min_sol_amount_out: u64,
  ) -> Result<()> {
    processors::sell::exec(ctx, token_amount, min_sol_amount_out)
  }
}
