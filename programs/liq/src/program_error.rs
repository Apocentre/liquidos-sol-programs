use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Only owner")]
  OnlyOwner,
  #[msg("Wrong deployer")]
  WrongDeployer,
  #[msg("Only Liquidos Curve Program")]
  OnlyLiquidosCurveProgram,
}
