use std::str::FromStr;
use anchor_lang::prelude::Pubkey;

// We add a bit of margin to each account data. This is helpful if in the future we add new fields to the existing structs.
// With this additional space we would not need to send `realloc` instructions which are inconvenient.
pub const SPACE_MARGIN: usize = 1000;

pub fn allowed_deployer() -> Pubkey {
  #[cfg(not(feature = "localnet"))]
  return Pubkey::from_str("DxVMyJ9YGahVLDXwEb5RaWcFx89JcAErCYGTJrPrneiw").unwrap();

  #[cfg(feature = "localnet")]
  return Pubkey::from_str("85Wgv3aHVyrZpMzmyCvd47hNC4g3f25SwJnboDksU86X").unwrap();
}
