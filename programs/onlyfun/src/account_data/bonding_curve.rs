
use std::mem::size_of;
use anchor_lang::prelude::*;
use rust_decimal::prelude::*;

pub const MAX_OPERATORS: usize = 5;

#[account]
pub struct BondingCurve {
  /// Total supply of the token
  pub total_supply: u64,
  /// The balance of reserve token i.e. SOL
  pub reserve_token_balance: u64,
}

impl BondingCurve {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();

  /// Calculates the number of tokens to mint based on the given amount of reserve tokens.
  /// This function is used when user buys the token with SOL
  pub fn calculate_purchase_return() -> Decimal {
    todo!()
  }

  /// Given an amount of tokens, calucates the amount of reserve tokens to be sent back.
  /// This function is used when user sells the tokens and receives back SOL
  pub fn calculate_sale_return() -> Decimal {
    todo!()
  }
}
