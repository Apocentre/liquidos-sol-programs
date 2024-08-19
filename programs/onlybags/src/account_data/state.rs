
use std::mem::size_of;
use anchor_lang::prelude::*;


#[account]
pub struct State {
  /// The owner that can handle various admin related teasks
  pub owner: Pubkey,
  /// The treasury account that receives fees
  pub treasury: Pubkey,
  /// The state of the staking program
  pub staking_program_state: Option<Pubkey>,
  /// Current protocol fees (fixed lamports amount). This is applied when the pool is created on Raydium
  pub protocol_fee: u64,
  /// Current trade fees (BPS). This is applied on each trade that takes place. Fees collected in SOL
  pub trade_fee_bps: u64,
  /// Current creator fees (fixed lamports amount). This is applied when the pool is created on Raydium
  pub creator_fee: u64,
  /// The total supply of the newly created tokens in the lowest denomination i.e. decimals included
  pub total_token_supply: u64,
  /// Staking allocation (BPS). This percentage of the total allocation will be distributed though the staking program
  pub staking_allocation_bps: u64,
}

impl State {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();

  pub fn new(
    owner: Pubkey,
    treasury: Pubkey,
    protocol_fee: u64,
    trade_fee_bps: u64,
    creator_fee: u64,
    total_token_supply: u64,
    staking_allocation_bps: u64,
  ) -> Self {
    Self {
      owner,
      treasury,
      staking_program_state: None,
      protocol_fee,
      trade_fee_bps,
      creator_fee,
      total_token_supply,
      staking_allocation_bps,
    }
  }
}
