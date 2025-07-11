use std::f64::consts::E;

use anchor_lang::prelude::*;
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;
use crate::math::decimal_error::DecimalErrorHandler;
use super::{constants::{LAMPORT_IN_SOL, ONE_TOKEN}, curve_formula::CurveFormula};

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct CurveV1;

impl CurveFormula for CurveV1 {
  fn calc_price(circulating_supply: u64) -> Result<u64> {
    let p0 = dec!(1.103).safe_mul(dec!(10).safe_powd(dec!(-8))?)?;
    let k = dec!(4.8235).safe_mul(dec!(10).safe_powd(dec!(-9))?)?;
    let circulating_supply = Self::normalize_token_amount(circulating_supply)?;

    let p = E.powf(
      k.safe_mul(circulating_supply)?
      .safe_to_f64()?
    );
    let p = Decimal::safe_from_f64(p)?
    .safe_mul(p0)?
    .safe_mul(LAMPORT_IN_SOL)?
    .safe_to_u64()?;

    Ok(p)
  }

  fn process_purchase_return(reserve_tokens_received: u64, circulating_supply: u64) -> Result<u64> {
    // divide by 10e9 to convert lamports to SOL
    let reserve_tokens_received = Self::normalize_sol_amount(reserve_tokens_received)?;

    let p0 = dec!(1.103).safe_mul(dec!(10).safe_powd(dec!(-8))?)?;
    let k = dec!(4.8235).safe_mul(dec!(10).safe_powd(dec!(-9))?)?;
    
    // divide by 10e6 to convert token amount to the highest denomination
    let circulating_supply = Self::normalize_token_amount(circulating_supply)?;
    let term_exp = Decimal::safe_from_f64(
      E.powf(k.safe_mul(circulating_supply)?
      .safe_to_f64()?)
    )?;
    let term = k.safe_mul(reserve_tokens_received)?.safe_div(p0.safe_mul(term_exp)?)?;
    let term_2 = dec!(1).safe_add(term)?;

    let tokens_amount = dec!(1).safe_div(k)?
    .safe_mul(term_2.ln())?
    .safe_mul(ONE_TOKEN)?
    .safe_to_u64()?;
    
    Ok(tokens_amount)
  }

  fn process_sale_return(token_amount: u64, circulating_supply: u64) -> Result<u64> {
    let p0 = dec!(1.103).safe_mul(dec!(10).safe_powd(dec!(-8))?)?;
    let k = dec!(4.8235).safe_mul(dec!(10).safe_powd(dec!(-9))?)?;
    let circulating_supply = Self::normalize_token_amount(circulating_supply)?;
    let token_amount = Self::normalize_token_amount(token_amount)?;

    let term_exp =  Decimal::safe_from_f64(
      E.powf(k.safe_mul(circulating_supply)?
      .safe_to_f64()?)
    )?;
    let term_exp_2 =  Decimal::safe_from_f64(
      E.powf(
        k.safe_mul(circulating_supply.safe_sub(token_amount)?)?
        .safe_to_f64()?
      )
    )?;

    let reserve_tokens_returned = p0.safe_div(k)?
    .safe_mul(term_exp.safe_sub(term_exp_2)?)?
    .safe_mul(LAMPORT_IN_SOL)?
    .safe_to_u64()?;

    Ok(reserve_tokens_returned)
  }
}

#[cfg(test)]
mod test {
  use crate::curve_formulas::{curve_formula::CurveFormula, curve_v1::CurveV1};
  use anchor_safe_math::SafeMath;

  // TODO: is this still relevant
  #[test]
  fn process_purchase_return_specific_values() {
    let reserve_tokens_received = 323;
    let reserve_token_balance = 89799992492_u64;
    let curve_max_accepted_amount = 89800000000 - reserve_token_balance;
    println!("curve_max_accepted_amount {}", curve_max_accepted_amount);
    let trade_fee_bps = 100;
    
    let max_accepted_amount = curve_max_accepted_amount
    .safe_mul(10_000).unwrap()
    .safe_div(10_000 - trade_fee_bps).unwrap();
    println!("max_accepted_amount {}", max_accepted_amount);

    let spendable_amount = u64::min(max_accepted_amount, reserve_tokens_received);
    println!("spendable_amount {}", spendable_amount);

    let trade_fees = spendable_amount
    .safe_mul(trade_fee_bps).unwrap()
    .safe_div(10_000).unwrap();

    let net_amount = spendable_amount.safe_sub(trade_fees).unwrap();
    println!("Net Amount {}", net_amount);

    let circulating_supply = 793004666216429;
    let k = CurveV1::process_purchase_return(net_amount, circulating_supply).unwrap();
    let circulating_supply = circulating_supply.safe_add(k).unwrap();
    let reserve_token_balance = reserve_token_balance.safe_add(reserve_tokens_received).unwrap();
    println!("reserve_token_balance {}", reserve_token_balance);

    println!("{}", CurveV1::calc_price(circulating_supply).unwrap());
  }
}
