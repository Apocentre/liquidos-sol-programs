use std::str::FromStr;

use anchor_lang::solana_program::pubkey::Pubkey;
use borsh::BorshSerialize;

pub fn id() -> Pubkey {
  #[cfg(not(feature = "devnet"))]
  return Pubkey::from_str("CPMDWBwJDtYax9qW7AyRuVC19Cc4L4Vcy4n2BHAbHkCW").unwrap();

  #[cfg(feature = "devnet")]
  return Pubkey::from_str("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C").unwrap();
}

#[derive(BorshSerialize)]
pub struct InitializeIx {
  pub init_amount_0: u64,
  pub init_amount_1: u64,
  pub open_time: u64,
}
