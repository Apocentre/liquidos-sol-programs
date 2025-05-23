use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only owner")]
  OnlyOwner,
  #[msg("Invalid staking token")]
  InvalidStakingToken,
  #[msg("Insufficient withdraw amount")]
  InsufficientWithdrawAmount,
}
