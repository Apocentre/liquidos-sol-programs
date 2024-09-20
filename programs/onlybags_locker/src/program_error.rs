use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Lock expired")]
  LockExpired,
}
