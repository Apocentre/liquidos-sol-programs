use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only owner")]
  OnlyOwner,
  #[msg("Slippage violation")]
  SlippageViolation,
  #[msg("Wrong rreasury")]
  WrongTreasury,
  #[msg("Curve closed")]
  CurveClosed,
  #[msg("Curve not complete")]
  CurveNotComplete,
  #[msg("Invalid curve token")]
  InvalidCurveToken,
}
