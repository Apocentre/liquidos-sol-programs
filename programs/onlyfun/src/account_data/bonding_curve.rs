
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

  const ONE_TOKEN: Decimal = dec!(1_000_000);
  const SOL_LAMPORT: Decimal = dec!(1_000_000_000);

  /// Calculates the number of tokens to mint based on the given amount of reserve tokens.
  /// This function is used when user buys the token with SOL
  pub fn calculate_purchase_return(&self, reserve_tokens_received: u64) -> Result<u64> {
    // divide by 10e9 to convert lamports to SOL
    let reserve_tokens_received: Decimal = Decimal::safe_from_u64(reserve_tokens_received)?.safe_div(Self::SOL_LAMPORT)?;

    let a = dec!(3.34315523).safe_mul(dec!(10).safe_powd(dec!(-9))?)?;
    let b = dec!(17.5970429);
    let c = dec!(299215564.8);
    // divide by 10e6 to convert token amount to the highest denomination
    let total_supply: Decimal = Decimal::safe_from_u64(self.total_supply)?.safe_div(Self::ONE_TOKEN)?;
    let d = a.safe_mul(total_supply)?.safe_sub(b)?.to_f64().unwrap();
    let e = std::f64::consts::E.powf(d);
    let e = Decimal::from_f64(e).unwrap();
    
    let k = reserve_tokens_received.safe_div(c)?
    .safe_add(e)?
    .safe_ln()?
    .safe_add(b)?
    .safe_div(a)?
    .safe_sub(total_supply)?
    .safe_mul(Self::ONE_TOKEN)?
    .safe_to_u64()?;

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

#[cfg(test)]
mod tests {
    use super::BondingCurve;

  #[test]
  fn returns_correct_purchase_amount() {
    let curve = BondingCurve {
      total_supply: 0,
      reserve_token_balance: 0,
    };

    let received = curve.calculate_purchase_return(89800000000).unwrap();
    println!("received >>>>>>>>>>>>>> {:?}", received);
    panic!()
  }
}
