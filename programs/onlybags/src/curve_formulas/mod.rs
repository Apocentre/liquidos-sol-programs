pub mod curve_v1;
pub mod curve_formula;
pub mod constants;

use std::ops::Deref;
use anchor_lang::prelude::*;
use curve_formula::CurveFormula;
use curve_v1::CurveV1;

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub enum CurveType {
  CurveV1(CurveV1),
  CurveV2,
}

impl Deref for CurveType {
  type Target = Box<dyn CurveFormula>;

  fn deref(&self) -> &Self::Target {
    match self {
      CurveType::CurveV1(curve_v1) => curve_v1,
      CurveType::CurveV2(curve_v2) => curve_v2
    }
  }
}
