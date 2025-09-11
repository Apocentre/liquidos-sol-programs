
use std::mem::size_of;
use anchor_lang::prelude::*;
use crate::program_error::ErrorCode;

#[account]
pub struct State {
  /// The owner that can handle various admin related tasks
  pub owner: Pubkey,
  /// The treasury account that receives fees and the corresponding trade fees (BPS).
  /// This is applied on each trade that takes place. Fees collected in SOL
  pub treasuries: Vec<(Pubkey, u64)>,
  /// Current protocol fees (fixed lamports amount). This is applied when the pool is created on Raydium
  pub protocol_fee: u64,
  /// Current creator fees (fixed lamports amount). This is applied when the pool is created on Raydium
  pub creator_fee: u64,
  /// The total supply of the newly created tokens in the lowest denomination i.e. decimals included
  pub total_token_supply: u64,
  /// The staking program Id
  pub staking_program: Option<Pubkey>,
  /// The state of the staking program
  pub staking_program_state: Option<Pubkey>,
  /// Staking allocation. The exact amount that will be distributed though the staking program
  pub staking_allocation: u64,
}

impl State {
  pub const MAX_TREASURY_ACCOUNTS: usize = 5;
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>()
  + Self::MAX_TREASURY_ACCOUNTS * size_of::<Pubkey>();

  pub fn new(
    owner: Pubkey,
    treasuries: Vec<(Pubkey, u64)>,
    protocol_fee: u64,
    creator_fee: u64,
    total_token_supply: u64,
    staking_allocation: u64,
  ) -> Result<Self> {
    let total_trade_fees: u64 = treasuries.iter().map(|(_, trade_fee_bps)| trade_fee_bps).sum();
    require!(total_trade_fees == 10_000, ErrorCode::TradeFeesMisconfiguration);

    Ok(Self {
      owner,
      treasuries,
      protocol_fee,
      creator_fee,
      total_token_supply,
      staking_program: None,
      staking_program_state: None,
      staking_allocation,
    })
  }
}
