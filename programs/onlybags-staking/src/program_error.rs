use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Invalid staking token")]
  InvalidStakingToken,
  #[msg("Staking token not set yet")]
  StakingTokenNotSet,
  #[msg("Invalid treasury account")]
  InvalidTreasury,
}
