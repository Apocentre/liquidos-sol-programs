pub mod curve_v1;
pub mod curve_v2;
pub mod curve_formula;
pub mod constants;

use anchor_lang::prelude::*;
use curve_formula::CurveFormula;
use curve_v1::CurveV1;
use curve_v2::CurveV2;
use crate::program_error::ErrorCode;

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub enum CurveType {
  CurveV1,
  CurveV2,
}

impl TryFrom<u8> for CurveType {
  type Error = Error;

  fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
    match value {
      1 => Ok(CurveType::CurveV1),
      2 => Ok(CurveType::CurveV2),
      _ => Err(error!(ErrorCode::CurveTypeNotSupported)),
    }
  }
}

impl CurveType {
  pub fn calc_price(&self, circulating_supply: u64) -> Result<u64> {
    match self {
      CurveType::CurveV1 => CurveV1::calc_price(circulating_supply),
      CurveType::CurveV2 => CurveV2::calc_price(circulating_supply),
    }
  }

  pub fn process_purchase_return(&self, reserve_tokens_received: u64, circulating_supply: u64) -> Result<u64> {
    match self {
      CurveType::CurveV1 => CurveV1::process_purchase_return(reserve_tokens_received, circulating_supply),
      CurveType::CurveV2 => CurveV2::process_purchase_return(reserve_tokens_received, circulating_supply),
    }
  }

  pub fn process_sale_return(&self, token_amount: u64, circulating_supply: u64) -> Result<u64> {
    match self {
      CurveType::CurveV1 => CurveV1::process_sale_return(token_amount, circulating_supply),
      CurveType::CurveV2 => CurveV2::process_sale_return(token_amount, circulating_supply),
    }
  }
}
