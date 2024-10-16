use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only owner")]
  OnlyOwner,
  #[msg("UserInfo initialized")]
  UserInfoInitialized,
  #[msg("Invalid staking token")]
  InvalidStakingToken,
  #[msg("Invalid treasury account")]
  InvalidTreasury,
  #[msg("Pool ended")]
  PoolEnded,
  #[msg("Insufficient withdraw amount")]
  InsufficientWithdrawAmount,
}
