use anchor_lang::prelude::*;
use rust_decimal::Decimal;
use crate::math::decimal_error::DecimalErrorHandler;

use super::constants::{LAMPORT_IN_SOL, ONE_TOKEN};

pub trait CurveFormula {
  /// Finds the current price of the curve
  fn calc_price(circulating_supply: u64) -> Result<u64> where Self: Sized;

  /// Calculates the number of tokens to mint based on the given amount of reserve tokens.
  /// This function is used when user buys the token with SOL
  fn process_purchase_return(reserve_tokens_received: u64, circulating_supply: u64) -> Result<u64> where Self: Sized;

  /// Given an amount of tokens, calucates the amount of reserve tokens to be sent back.
  /// This function is used when user sells the tokens and receives back SOL
  fn process_sale_return(token_amount: u64, circulating_supply: u64) -> Result<u64> where Self: Sized;

  fn normalize_token_amount(amount: u64) -> Result<Decimal>
    where Self: Sized 
{
    let value = Decimal::safe_from_u64(amount)?.safe_div(ONE_TOKEN)?;
    Ok(value)
  }

  fn normalize_sol_amount(amount: u64) -> Result<Decimal>
    where Self: Sized
  {
    let value = Decimal::safe_from_u64(amount)?.safe_div(LAMPORT_IN_SOL)?;
    Ok(value)
  }
}
