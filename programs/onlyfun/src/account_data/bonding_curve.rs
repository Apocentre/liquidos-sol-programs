
use std::mem::size_of;
use anchor_lang::prelude::*;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use crate::math::decimal_error::DecimalErrorHandler;

pub const MAX_OPERATORS: usize = 5;

#[account]
pub struct BondingCurve {
  /// Total supply of the token
  pub total_supply: u64,
  /// The balance of reserve token i.e. SOL
  pub reserve_token_balance: u64,
}

impl BondingCurve {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();

  /// Calculates the number of tokens to mint based on the given amount of reserve tokens.
  /// This function is used when user buys the token with SOL
  pub fn calculate_purchase_return(&self, reserve_tokens_received: u64) -> Result<Decimal> {
    let a = dec!(3.34315523).safe_mul(dec!(10).safe_powd(dec!(-9))?)?;
    let b = dec!(17.5970429);
    let c = dec!(299215564.8);
    let total_supply: Decimal = self.total_supply.into();
    let d = Decimal::E.safe_powd(a.safe_mul(total_supply)?.safe_sub(b)?)?;
    let reserve_tokens_received: Decimal = reserve_tokens_received.into();
    
    let k = reserve_tokens_received.safe_div(c)?
    .safe_add(d)?
    .safe_ln()?
    .safe_add(b)?
    .safe_div(a)?
    .safe_sub(total_supply)?;


    Ok(k)
  }

  /// Given an amount of tokens, calucates the amount of reserve tokens to be sent back.
  /// This function is used when user sells the tokens and receives back SOL
  pub fn calculate_sale_return(&self, tokens_sold: u64) -> Result<Decimal> {
    let a = dec!(3.34315523).safe_mul(dec!(10).safe_powd(dec!(-9))?)?;
    let b = dec!(17.5970429);
    let c = dec!(299215564.8);
    let total_supply: Decimal = self.total_supply.into();
    let tokens_sold: Decimal = tokens_sold.into();
    let d = Decimal::E.safe_powd(a.safe_mul(total_supply.safe_sub(tokens_sold)?.safe_sub(b)?)?)?;
    let e = Decimal::E.safe_powd(a.safe_mul(total_supply)?.safe_sub(b)?)?;

    let reserve_tokens_returned = c.safe_mul(d.safe_sub(e)?)?;

    Ok(reserve_tokens_returned) 
  }
}
