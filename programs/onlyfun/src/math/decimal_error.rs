use anchor_lang::prelude::*;
use rust_decimal::prelude::*;

#[error_code]
/// Errors that can be triggered by executing one of the supported numeric operations
pub enum ErrorCode {
  #[msg("Decimal error")]
  DecimalError,
}

pub trait DecimalErrorHandler {
  fn safe_add(&self, rhs: Decimal) -> Result<Decimal>;
  fn safe_sub(&self, rhs: Decimal) -> Result<Decimal>;
  fn safe_mul(&self, rhs: Decimal) -> Result<Decimal>;
  fn safe_powd(&self, rhs: Decimal) -> Result<Decimal>;
  fn safe_exp(&self) -> Result<Decimal>;
  fn safe_div(&self, rhs: Decimal) -> Result<Decimal>;
  fn safe_ln(&self) -> Result<Decimal>;
}

impl DecimalErrorHandler for Decimal {
  fn safe_add(&self, rhs: Decimal) -> Result<Decimal> {
    match self.checked_add(rhs) {
      Some(val) => Ok(val),
      None => Err(error!(ErrorCode::DecimalError)),
    }
  }

  fn safe_sub(&self, rhs: Decimal) -> Result<Decimal> {
    match self.checked_sub(rhs) {
      Some(val) => Ok(val),
      None => Err(error!(ErrorCode::DecimalError)),
    }
  }

  fn safe_mul(&self, rhs: Decimal) -> Result<Decimal> {
    match self.checked_mul(rhs) {
      Some(val) => Ok(val),
      None => Err(error!(ErrorCode::DecimalError)),
    }
  }

  fn safe_powd(&self, rhs: Decimal) -> Result<Decimal> {
    match self.checked_powd(rhs) {
      Some(val) => Ok(val),
      None => Err(error!(ErrorCode::DecimalError)),
    }
  }

  fn safe_exp(&self) -> Result<Decimal> {
    match self.checked_exp() {
      Some(val) => Ok(val),
      None => Err(error!(ErrorCode::DecimalError)),
    }
  }

  fn safe_div(&self, rhs: Decimal) -> Result<Decimal> {
    match self.checked_div(rhs) {
      Some(val) => Ok(val),
      None => Err(error!(ErrorCode::DecimalError)),
    }
  }

  fn safe_ln(&self) -> Result<Decimal> {
    match self.checked_ln() {
      Some(val) => Ok(val),
      None => Err(error!(ErrorCode::DecimalError)),
    }
  }
}
