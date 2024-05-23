use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only owner")]
  OnlyOwner,
  #[msg("Slippage violation")]
  SlippageViolation,
  #[msg("Wrong rreasury")]
  WrongTreasury,

}
