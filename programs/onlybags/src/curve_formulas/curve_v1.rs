use anchor_lang::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use math::decimal_error::DecimalErrorHandler;
use super::{constants::{LAMPORT_IN_SOL, ONE_TOKEN}, curve_formula::CurveFormula};

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct CurveV1;

impl CurveFormula for CurveV1 {
  fn calc_price(circulating_supply: u64) -> Result<u64> {
    let a = dec!(3.34315523).safe_mul(dec!(10).safe_powd(dec!(-9))?)?;
    let b = dec!(17.5970429);
    let circulating_supply = Self::normalize_token_amount(circulating_supply)?;

    let p = std::f64::consts::E.powf(a.safe_mul(circulating_supply)?.safe_sub(b)?.safe_to_f64()?);
    let p = Decimal::safe_from_f64(p)?
    .safe_mul(LAMPORT_IN_SOL)?
    .safe_to_u64()?;

    Ok(p)
  }

  fn process_purchase_return(reserve_tokens_received: u64, circulating_supply: u64) -> Result<u64> {
    // divide by 10e9 to convert lamports to SOL
    let reserve_tokens_received_sol = Self::normalize_sol_amount(reserve_tokens_received)?;

    let a = dec!(3.34315523).safe_mul(dec!(10).safe_powd(dec!(-9))?)?;
    let b = dec!(17.5970429);
    let c = dec!(299215564.8);
    // divide by 10e6 to convert token amount to the highest denomination
    let circulating_supply = Self::normalize_token_amount(circulating_supply)?;
    let d = a.safe_mul(circulating_supply)?.safe_sub(b)?.safe_to_f64()?;
    let e = std::f64::consts::E.powf(d);
    let e = Decimal::safe_from_f64(e)?;
    
    let k = reserve_tokens_received_sol.safe_div(c)?
    .safe_add(e)?
    .safe_ln()?
    .safe_add(b)?
    .safe_div(a)?
    .safe_sub(circulating_supply)?
    .safe_mul(ONE_TOKEN)?
    .safe_to_u64()?;
    
    Ok(k)
  }

  fn process_sale_return(token_amount: u64, circulating_supply: u64) -> Result<u64> {
    let a = dec!(3.34315523).safe_mul(dec!(10).safe_powd(dec!(-9))?)?;
    let b = dec!(17.5970429);
    let c = dec!(299215564.8);
    let circulating_supply = Self::normalize_token_amount(circulating_supply)?;
    let token_amount_normalized = Self::normalize_token_amount(token_amount)?;

    let d = Decimal::safe_from_f64(std::f64::consts::E.powf(
      a.safe_mul(circulating_supply.safe_sub(token_amount_normalized)?)?.safe_sub(b)?.safe_to_f64()?
    ))?;

    let e = Decimal::safe_from_f64(std::f64::consts::E.powf(
      a.safe_mul(circulating_supply)?.safe_sub(b)?.safe_to_f64()?
    ))?;

    let reserve_tokens_returned = c.safe_mul(d.safe_sub(e)?)?
    .safe_mul(dec!(-1))?
    .safe_mul(LAMPORT_IN_SOL)?
    .safe_to_u64()?;

    Ok(reserve_tokens_returned)
  }
}

#[cfg(test)]
mod test {
  use crate::curve_formulas::{curve_formula::CurveFormula, curve_v1::CurveV1};
  use anchor_safe_math::SafeMath;

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
