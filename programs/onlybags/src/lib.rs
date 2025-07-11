#![allow(unexpected_cfgs)]
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
  update_state::*, create_staking_pool::*, resize_state::*, resize_bonding_curve::*,
};

declare_id!("HUGG29eLrsEjGyedD56AdFnZgejSJ7HuEB4wmNsLFfGV");

#[program]
pub mod onlybags {
  use super::*;

  /// Initialize
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `treasury` - The treasury account that receives fees
  /// * `protocol_fee` - Current protocol fees (fixed lamports amount). This is applied when the pool is created on Raydium
  /// * `trade_fee_bps` - Current trade fees (BPS). This is applied on each trade that takes place. Fees collected in SOL
  /// * `creator_fee` - Current creator fees (fixed lamports amount). This is applied when the pool is created on Raydium
  /// * `total_token_supply` - The total supply of the newly created tokens in the lowest denomination i.e. decimals included
  /// * `staking_allocation` - Staking allocation. The exact amount that will be distributed though the staking program
  pub fn initialize(
    ctx: Context<Initialize>,
    treasury: Pubkey,
    protocol_fee: u64,
    trade_fee_bps: u64,
    creator_fee: u64,
    total_token_supply: u64,
    staking_allocation: u64,
  ) -> Result<()> {
    processors::initialize::exec(
      ctx,
      treasury,
      protocol_fee,
      trade_fee_bps,
      creator_fee,
      total_token_supply,
      staking_allocation,
    )
  }

  /// UpdateState
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `staking_program` - The staking program Id
  /// * `staking_program_state` - The state of the staking program
  /// * `protocol_fee` - Current protocol fees (fixed lamports amount). This is applied when the pool is created on Raydium
  /// * `trade_fee_bps` - Current trade fees (BPS). This is applied on each trade that takes place. Fees collected in SOL
  /// * `creator_fee` - Current creator fees (fixed lamports amount). This is applied when the pool is created on Raydium
  /// * `total_token_supply` - The total supply of the newly created tokens in the lowest denomination i.e. decimals included
  /// * `staking_allocation` - Staking allocation. The exact amount that will be distributed though the staking program
  pub fn update_state(
    ctx: Context<UpdateState>,
    staking_program: Pubkey,
    staking_program_state: Pubkey,
    protocol_fee: u64,
    trade_fee_bps: u64,
    creator_fee: u64,
    total_token_supply: u64,
    staking_allocation: u64,
  ) -> Result<()> {
    processors::update_state::exec(
      ctx,
      staking_program,
      staking_program_state,
      protocol_fee,
      trade_fee_bps,
      creator_fee,
      total_token_supply,
      staking_allocation,

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

  /// CreateStakingPool
  ///
  /// Creates a new staking pool
  /// 
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  pub fn create_staking_pool(ctx: Context<CreateStakingPool>) -> Result<()> {
    processors::create_staking_pool::exec(ctx)
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

  /// ResizeState
  /// 
  /// Allows admin to resize the size of the state account
  /// 
  /// # Arguments
  /// 
  /// * `_ctx` - The Anchor context holding the accounts
  /// * `_size` - The new size of the account. Note! don't pass a smaller size than the current one
  ///             because it will end up is loss of data. Check the size manually on the explorer and pass
  ///             a higher value.
  pub fn resize_state(_ctx: Context<ResizeState>, _size: u64) -> Result<()> {
    Ok(())
  }

  /// ResizeBondingCurve
  /// 
  /// Allows admin to resize the size of the bonding_curve account
  /// 
  /// # Arguments
  /// 
  /// * `_ctx` - The Anchor context holding the accounts
  /// * `_size` - The new size of the account. Note! don't pass a smaller size than the current one
  ///             because it will end up is loss of data. Check the size manually on the explorer and pass
  ///             a higher value.
  pub fn resize_bonding_curve(_ctx: Context<ResizeBondingCurve>, _size: u64) -> Result<()> {
    Ok(())
  }
}
