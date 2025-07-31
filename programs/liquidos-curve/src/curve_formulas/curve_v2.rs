use anchor_lang::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use math::decimal_error::DecimalErrorHandler;
use super::{constants::{LAMPORT_IN_SOL, ONE_TOKEN}, curve_formula::CurveFormula};

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct CurveV2;

impl CurveFormula for CurveV2 {
  fn calc_price(circulating_supply: u64) -> Result<u64> {
    let m = dec!(1.777777778).safe_mul(dec!(10).safe_powd(dec!(-15))?)?;
    let circulating_supply = Self::normalize_token_amount(circulating_supply)?;

    let p = m.safe_mul(circulating_supply)?
    .safe_mul(LAMPORT_IN_SOL)?
    .safe_to_u64()?;

    Ok(p)
  }

  fn process_purchase_return(reserve_tokens_received: u64, circulating_supply: u64) -> Result<u64> {
    let m = dec!(1.777777778).safe_mul(dec!(10).safe_powd(dec!(-15))?)?;
    // divide by 10e9 to convert lamports to SOL
    let reserve_tokens_received = Self::normalize_sol_amount(reserve_tokens_received)?;
    let circulating_supply = Self::normalize_token_amount(circulating_supply)?;
    let term = circulating_supply.safe_powd(dec!(2))?
    .safe_add(
      dec!(2)
      .safe_mul(reserve_tokens_received)?
      .safe_div(m)?
    )?;
    let term_sqrt = Decimal::safe_from_f64(
      term.safe_to_f64()?.sqrt()
    )?;

    let k = term_sqrt
    .safe_sub(circulating_supply)?
    .safe_mul(ONE_TOKEN)?
    .safe_to_u64()?;

    Ok(k)
  }

  fn process_sale_return(token_amount: u64, circulating_supply: u64) -> Result<u64> {
    let m = dec!(1.777777778).safe_mul(dec!(10).safe_powd(dec!(-15))?)?;
    let circulating_supply = Self::normalize_token_amount(circulating_supply)?;
    let token_amount = Self::normalize_token_amount(token_amount)?;

    let reserve_tokens_returned = m.safe_mul(token_amount)?
    .safe_mul(
      circulating_supply.safe_sub(
        dec!(0.5).safe_mul(token_amount)?
      )?
    )?
    .safe_mul(LAMPORT_IN_SOL)?
    .safe_to_u64()?;

    Ok(reserve_tokens_returned)
  }
}
