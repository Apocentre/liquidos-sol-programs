use std::str::FromStr;
use anchor_lang::prelude::{borsh::BorshSerialize, *};

pub fn id() -> Pubkey {
  #[cfg(not(feature = "devnet"))]
  return Pubkey::from_str("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C").unwrap();

  #[cfg(feature = "devnet")]
  return Pubkey::from_str("DRaycpLY18LhpbydsBWbVJtxpNv9oXPgjRSfpF2bWpYb").unwrap();
}

pub fn amm_config() -> Pubkey {
  #[cfg(not(feature = "devnet"))]
  return Pubkey::from_str("D4FPEruKEHrG5TenZ2mpDGEfu1iUvTiqBxvpU8HLBvC2").unwrap();

  #[cfg(feature = "devnet")]
  return Pubkey::from_str("5MxLgy9oPdTC3YgkiePHqr3EoCRD9uLVYRQS2ANAs7wy").unwrap();
}

pub fn is_wsol(other: &Pubkey) -> Result<bool> {
  let wsol = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
  Ok(other.eq(&wsol))
}

#[derive(BorshSerialize)]
pub struct SwapBaseInputIx {
  pub amount_in: u64,
  pub minimum_amount_out: u64,
}


#[derive(BorshSerialize)]
pub struct SwapBaseOutputIx {
  pub max_amount_in: u64,
  pub amount_out_less_fee: u64,
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
  /// The pool creator fee, denominated in hundredths of a bip (10^-6)
  pub creator_fee_rate: u64,
  /// padding
  pub padding: [u64; 15],
}
