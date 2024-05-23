
use std::mem::size_of;
use anchor_lang::prelude::*;

pub const MAX_OPERATORS: usize = 5;

#[account]
pub struct State {
  /// The owner that can handle various admin related teasks
  pub owner: Pubkey,
  /// The list of all operators that can run admin related tasks
  pub operators: Vec<Pubkey>,
  /// Current target of SOL each pool should receive
  pub sol_target: u64,
  /// Current protocol fees (BPS)
  pub protocol_fee_bps: u64,
  /// Bump of the cpi_authority PDA
  pub cpi_authority_bump: u8,
}

impl State {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>()
  + size_of::<Pubkey>() * MAX_OPERATORS;

  pub fn new(
    owner: Pubkey,
    operators: Vec<Pubkey>,
    sol_target: u64,
    protocol_fee_bps: u64,
    cpi_authority_bump: u8,
  ) -> Self {
    Self {
      owner,
      operators,
      sol_target,
      protocol_fee_bps,
      cpi_authority_bump,
    }
  }
}
