use std::str::FromStr;
use anchor_lang::prelude::{borsh::BorshSerialize, *};

pub fn id() -> Pubkey {
  #[cfg(not(feature = "devnet"))]
  return Pubkey::from_str("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C").unwrap();

  #[cfg(feature = "devnet")]
  return Pubkey::from_str("CPMDWBwJDtYax9qW7AyRuVC19Cc4L4Vcy4n2BHAbHkCW").unwrap();
}

pub fn amm_config() -> Pubkey {
  #[cfg(not(feature = "devnet"))]
  return Pubkey::from_str("D4FPEruKEHrG5TenZ2mpDGEfu1iUvTiqBxvpU8HLBvC2").unwrap();

  #[cfg(feature = "devnet")]
  return Pubkey::from_str("9zSzfkYy6awexsHvmggeH36pfVUdDGyCcwmjT3AQPBj6").unwrap();
}

/// The last buyer will have to pay for the rent of the accounts created by the Raydium CP swap program.
/// This amount will have to come from the curves balance. So last buyer must be funded with this amount.
pub const RENT_COST: u64 = 50000000;

#[derive(BorshSerialize)]
pub struct InitializeIx {
  pub init_amount_0: u64,
  pub init_amount_1: u64,
  pub open_time: u64,
}

/// Holds the current owner of the factory
#[account]
pub struct AmmConfig {
  /// Bump to identify PDA
  pub bump: u8,
  /// Status to control if new pool can be create
  pub disable_create_pool: bool,
  /// Config index
  pub index: u16,
  /// The trade fee, denominated in hundredths of a bip (10^-6)
  pub trade_fee_rate: u64,
  /// The protocol fee
  pub protocol_fee_rate: u64,
  /// The fund fee, denominated in hundredths of a bip (10^-6)
  pub fund_fee_rate: u64,
  /// Fee for create a new pool
  pub create_pool_fee: u64,
  /// Address of the protocol fee owner
  pub protocol_owner: Pubkey,
  /// Address of the fund fee owner
  pub fund_owner: Pubkey,
  /// padding
  pub padding: [u64; 16],
}
