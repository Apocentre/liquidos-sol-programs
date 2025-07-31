use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only owner")]
  OnlyOwner,
  #[msg("Invalid staking token")]
  InvalidStakingToken,
  #[msg("Invalid treasury account")]
  InvalidTreasury,
  #[msg("Withdraw is locked")]
  WithdrawLock,
  #[msg("Pool has not started")]
  PoolNotStarted,
  #[msg("Pool ended")]
  PoolEnded,
  #[msg("Insufficient withdraw amount")]
  InsufficientWithdrawAmount,
}
