use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Lock expired")]
  LockExpired,
  #[msg("Lock not expired")]
  LockNotExpired,
}
