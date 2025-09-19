use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only owner")]
  OnlyOwner,
  #[msg("Wrong treasury")]
  WrongTreasury,
  #[msg("Trade fees must sum to 10000")]
  TradeFeesMisconfiguration,
}
