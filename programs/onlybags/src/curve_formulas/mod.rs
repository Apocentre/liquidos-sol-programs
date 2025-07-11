pub mod curve_v1;
pub mod curve_v2;
pub mod curve_v3;
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
  CurveV3,
}

impl From<&CurveType> for u8 {
  fn from(value: &CurveType) -> Self {
    match value {
      CurveType::CurveV1 => 1,
      CurveType::CurveV2 => 2,
      CurveType::CurveV3 => 3,
    }
  }
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
  pub fn sol_target(&self) -> u64 {
    match self {
      CurveType::CurveV1 => 82891351000, // 82.891351 SOL
      CurveType::CurveV2 => 500000000000, // 500 SOL
      CurveType::CurveV3 => 40014855000, // 40.014855 SOL
    }
  }

  pub fn calc_price(&self, circulating_supply: u64) -> Result<u64> {
    match self {
      CurveType::CurveV1 => CurveV1::calc_price(circulating_supply),
      CurveType::CurveV2 => CurveV2::calc_price(circulating_supply),
      CurveType::CurveV3 => CurveV3::calc_price(circulating_supply),
    }
  }

  pub fn process_purchase_return(&self, reserve_tokens_received: u64, circulating_supply: u64) -> Result<u64> {
    match self {
      CurveType::CurveV1 => CurveV1::process_purchase_return(reserve_tokens_received, circulating_supply),
      CurveType::CurveV2 => CurveV2::process_purchase_return(reserve_tokens_received, circulating_supply),
      CurveType::CurveV3 => CurveV3::process_purchase_return(reserve_tokens_received, circulating_supply),
    }
  }

  pub fn process_sale_return(&self, token_amount: u64, circulating_supply: u64) -> Result<u64> {
    match self {
      CurveType::CurveV1 => CurveV1::process_sale_return(token_amount, circulating_supply),
      CurveType::CurveV2 => CurveV2::process_sale_return(token_amount, circulating_supply),
      CurveType::CurveV3 => CurveV3::process_sale_return(token_amount, circulating_supply),
    }
  }
}
