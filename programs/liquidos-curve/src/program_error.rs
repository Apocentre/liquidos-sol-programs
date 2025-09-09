use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only owner")]
  OnlyOwner,
  #[msg("Slippage violation")]
  SlippageViolation,
  #[msg("Wrong treasury")]
  WrongTreasury,
  #[msg("Wrong token creator")]
  WrongTokenCreator,
  #[msg("Curve closed")]
  CurveClosed,
  #[msg("Curve not complete")]
  CurveNotComplete,
  #[msg("Invalid curve token")]
  InvalidCurveToken,
  #[msg("Wrong program id")]
  WrongProgramId,
  #[msg("Wrong instruction data")]
  WrongIxData,
  #[msg("Expected MoveLiquidity ix")]
  ExpectedMoveLiquidityIx,
  #[msg("Expected CreateStakingPool ix")]
  ExpectedCreateStakingPoolIx,
  #[msg("Curve type not supported")]
  CurveTypeNotSupported,
  #[msg("Wrong staking program")]
  WrongStakingProgram,
  #[msg("Wrong staking program state")]
  WrongStakingProgramState,
}
