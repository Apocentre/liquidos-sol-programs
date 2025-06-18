
use anchor_lang::prelude::*;
use crate::constants::SPACE_MARGIN;

#[account]
#[derive(InitSpace, Debug)]
pub struct BondingCurve {
  /// Current circulating supply of the token in the lowest denomination i.e. decimals included
  pub circulating_supply: u64,
  /// The balance of reserve token i.e. SOL in the lowest denomination (lamport) i.e. decimals included
  pub reserve_token_balance: u64,
  /// The PDA bump of this account
  pub bump: u8,
}

impl BondingCurve {
  pub const MAX_SIZE: usize = 8 + Self::INIT_SPACE + SPACE_MARGIN;
  pub const MAX_SUPPLY: u64 = 50_000_000_000_000_000; // 50M

  pub fn new(bump: u8) -> Self {
    Self {
      circulating_supply: 0,
      reserve_token_balance: 0,
      bump,
    }
  }
}
