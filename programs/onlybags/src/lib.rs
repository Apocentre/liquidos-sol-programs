pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;
pub mod math;
pub mod raydium;
pub mod curve_formulas;

use anchor_lang::prelude::*;
use crate::instructions::{
  initialize::*, create_token::*, create_tax_token::*, buy::*, sell::*, move_liquidity::*,
};

declare_id!("3vVWsiMcHXqacBY1ApXj7YFrpmob3uDGm3TTStcywKEn");

#[program]
pub mod onlybags {
  use super::*;

  /// Initialize
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `treasury` - The treasury account that receives fees
  /// * `sol_target` - Current target of SOL each pool should receive before it goes to the 
  /// * `protocol_fee` - Current protocol fees (fixed lamports amount). This is applied when the pool is created on Raydium
  /// * `trade_fee_bps` - Current trade fees (BPS). This is applied on each trade that takes place. Fees collected in SOL
  /// * `creator_fee` - Current creator fees (fixed lamports amount). This is applied when the pool is created on Raydium
  /// * `total_token_supply` - The total supply of the newly created tokens in the lowest denomination i.e. decimals included
  pub fn initialize(
    ctx: Context<Initialize>,
    treasury: Pubkey,
    sol_target: u64,
    protocol_fee: u64,
    trade_fee_bps: u64,
    creator_fee: u64,
    total_token_supply: u64,
  ) -> Result<()> {
    processors::initialize::exec(
      ctx,
      treasury,
      sol_target,
      protocol_fee,
      trade_fee_bps,
      creator_fee,
      total_token_supply,
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
  /// * `curve_type` - The type of the curve. The numner defines the version e.g. CurveV1 then curve_type = 1
  pub fn create_token(
    ctx: Context<CreateToken>,
    name: String,
    symbol: String,
    uri: String,
    curve_type: u8,
  ) -> Result<()> {
    processors::create_token::exec(ctx, name, symbol, uri, curve_type)
  }

  /// CreateToken
  ///
  /// Creates a new tax token
  /// 
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `name` - The name of the token (used in the metadata account)
  /// * `symbol` - The symbol of the token (used in the metadata account)
  /// * `uri` - The uri of the token (used in the metadata account)
  /// * `fee_bps` - Transfer fee BPS
  /// * `max_fee` - Max fee that can be applied
  /// * `curve_type` - The type of the curve. The numner defines the version e.g. CurveV1 then curve_type = 1
  pub fn create_tax_token(
    ctx: Context<CreateTaxToken>,
    name: String,
    symbol: String,
    uri: String,
    fee_bps: u16,
    max_fee: u64,
    curve_type: u8,
  ) -> Result<()> {
    processors::create_tax_token::exec(ctx, name, symbol, uri, fee_bps, max_fee, curve_type)
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
  pub fn buy<'info>(
    ctx: Context<'_, '_, '_, 'info, Buy<'info>>,
    amount: u64,
    min_amount_out: u64,
  ) -> Result<()> {
    processors::buy::exec(ctx, amount, min_amount_out)
  }

  /// MoveLiquidity
  ///
  /// Moves the liquidity to Raydium
  /// 
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  pub fn move_liquidity(ctx: Context<MoveLiquidity>) -> Result<()> {
    processors::move_liquidity::exec(ctx)
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
