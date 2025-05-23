use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only owner")]
  OnlyOwner,
  #[msg("Wrong deployer")]
  WrongDeployer,
  #[msg("Invalid staking token")]
  InvalidStakingToken,
  #[msg("Insufficient withdraw amount")]
  InsufficientWithdrawAmount,
  #[msg("Same block harvest")]
  SameBlockHarvest,
  #[msg("Round end")]
  RoundEnd,
}
