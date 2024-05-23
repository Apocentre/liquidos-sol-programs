
use std::mem::size_of;
use anchor_lang::prelude::*;

pub const MAX_OPERATORS: usize = 5;

#[account]
pub struct State {
  /// The owner that can handle various admin related teasks
  pub owner: Pubkey,
  /// The treasury account that receives fees
  pub treasury: Pubkey,
  /// Current target of SOL each pool should receive
  pub sol_target: u64,
  /// Current protocol fees (BPS). This is applied when the pool is created on Raydium
  pub protocol_fee_bps: u64,
  /// Current trade fees (BPS). This is applied on each trade that takes place. Fees collected in SOL
  pub trade_fee_bps: u64,
  /// Bump of the cpi_authority PDA
  pub cpi_authority_bump: u8,
}

impl State {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>()
  + size_of::<Pubkey>() * MAX_OPERATORS;

  pub fn new(
    owner: Pubkey,
    treasury: Pubkey,
    sol_target: u64,
    protocol_fee_bps: u64,
    trade_fee_bps: u64,
    cpi_authority_bump: u8,
  ) -> Self {
    Self {
      owner,
      treasury,
      sol_target,
      protocol_fee_bps,
      trade_fee_bps,
      cpi_authority_bump,
    }
  }
}
