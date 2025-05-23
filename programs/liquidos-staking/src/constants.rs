use std::str::FromStr;
use anchor_lang::prelude::Pubkey;

pub fn allowed_deployer() -> Pubkey {
  #[cfg(not(feature = "devnet"))]
  return Pubkey::from_str("DxVMyJ9YGahVLDXwEb5RaWcFx89JcAErCYGTJrPrneiw").unwrap();

  #[cfg(feature = "devnet")]
  return Pubkey::from_str("85Wgv3aHVyrZpMzmyCvd47hNC4g3f25SwJnboDksU86X").unwrap();
}

pub fn liquidos_token() -> Pubkey {
  #[cfg(not(feature = "devnet"))]
  return Pubkey::from_str("DxVMyJ9YGahVLDXwEb5RaWcFx89JcAErCYGTJrPrneiw").unwrap();

  #[cfg(feature = "devnet")]
  return Pubkey::from_str("85Wgv3aHVyrZpMzmyCvd47hNC4g3f25SwJnboDksU86X").unwrap();
}
