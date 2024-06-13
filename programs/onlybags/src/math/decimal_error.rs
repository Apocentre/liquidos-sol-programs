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
  fn safe_from_u64(rhs: u64)-> Result<Decimal>;
  fn safe_from_f64(rhs: f64)-> Result<Decimal>;
  fn safe_to_u64(&self)-> Result<u64>;
  fn safe_to_f64(&self)-> Result<f64>;
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
  
  fn safe_from_u64(rhs: u64)-> Result<Decimal> {
    match Decimal::from_u64(rhs) {
      Some(val) => Ok(val),
      None => Err(error!(ErrorCode::DecimalError)),
    }
  }
  
  fn safe_to_u64(&self)-> Result<u64> {
    match Decimal::to_u64(self) {
      Some(val) => Ok(val),
      None => Err(error!(ErrorCode::DecimalError)),
    }
  }
  
  fn safe_from_f64(rhs: f64)-> Result<Decimal> {
    match Decimal::from_f64(rhs) {
      Some(val) => Ok(val),
      None => Err(error!(ErrorCode::DecimalError)),
    }
  }
  
  fn safe_to_f64(&self)-> Result<f64> {
    match Decimal::to_f64(self) {
      Some(val) => Ok(val),
      None => Err(error!(ErrorCode::DecimalError)),
    }
  }
}
