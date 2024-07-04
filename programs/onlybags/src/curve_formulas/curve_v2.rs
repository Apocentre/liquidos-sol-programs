use anchor_lang::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::math::decimal_error::DecimalErrorHandler;
use super::{constants::{LAMPORT_IN_SOL, ONE_TOKEN}, curve_formula::CurveFormula};

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct CurveV2;

impl CurveFormula for CurveV2 {
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
