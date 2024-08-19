use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only owner")]
  OnlyOwner,
  #[msg("Invalid staking token")]
  InvalidStakingToken,
  #[msg("Staking token not set yet")]
  StakingTokenNotSet,
  #[msg("Invalid treasury account")]
  InvalidTreasury,
  #[msg("Pool ended")]
  PoolEnded,
  #[msg("Insufficient withdraw amount")]
  InsufficientWithdrawAmount,
}
