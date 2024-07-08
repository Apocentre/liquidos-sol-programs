use anchor_lang::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::math::decimal_error::DecimalErrorHandler;
use super::{constants::{LAMPORT_IN_SOL, ONE_TOKEN}, curve_formula::CurveFormula};

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct CurveV2;

impl CurveFormula for CurveV2 {
  fn calc_price(circulating_supply: u64) -> Result<u64> {
    let a = dec!(9.31).safe_mul(dec!(10).safe_powd(dec!(-16))?)?;
    let b = dec!(6.21).safe_mul(dec!(10).safe_powd(dec!(-8))?)?;
    let circulating_supply = Self::normalize_token_amount(circulating_supply)?;

    let p = a.safe_mul(circulating_supply)?
    .safe_add(b)?
    .safe_mul(LAMPORT_IN_SOL)?
    .safe_to_u64()?;

    Ok(p)
  }

  fn process_purchase_return(reserve_tokens_received: u64, circulating_supply: u64) -> Result<u64> {
    // divide by 10e9 to convert lamports to SOL
    let reserve_tokens_received_sol = Self::normalize_sol_amount(reserve_tokens_received)?;
    let circulating_supply = Self::normalize_token_amount(circulating_supply)?;

    let a = dec!(9.31).safe_mul(dec!(10).safe_powd(dec!(-16))?)?;
    let b = a.safe_mul(circulating_supply)?.safe_add(
      dec!(6.21).safe_mul(dec!(10).safe_powd(dec!(-8))?)?
    )?;
    let b_square = b.safe_powd(dec!(2))?;

    let c = b_square.safe_add(
      dec!(1.862).safe_mul(dec!(10).safe_powd(dec!(-15))?)?.safe_mul(reserve_tokens_received_sol)?
    )?;
    let c_root = Decimal::safe_from_f64(
      c.safe_to_f64()?.sqrt()
    )?;

    let k = b.safe_mul(dec!(-1))?
    .safe_add(c_root)?
    .safe_div(a)?
    .safe_mul(ONE_TOKEN)?
    .safe_to_u64()?;

    Ok(k)
  }

  fn process_sale_return(token_amount: u64, circulating_supply: u64) -> Result<u64> {
    let a = dec!(9.31).safe_mul(dec!(10).safe_powd(dec!(-16))?)?;
    let b = dec!(6.21).safe_mul(dec!(10).safe_powd(dec!(-8))?)?;
    let circulating_supply = Self::normalize_token_amount(circulating_supply)?;
    let token_amount_normalized = Self::normalize_token_amount(token_amount)?;
    let c = a.safe_mul(token_amount_normalized.safe_powd(dec!(2))?)?
    .safe_div(dec!(2))?;
    let d = b.safe_mul(token_amount_normalized)?;

    let reserve_tokens_returned = a.safe_mul(circulating_supply)?
    .safe_mul(token_amount_normalized)?
    .safe_sub(c)?
    .safe_add(d)?
    .safe_mul(LAMPORT_IN_SOL)?
    .safe_to_u64()?;

    Ok(reserve_tokens_returned)
  }
}
