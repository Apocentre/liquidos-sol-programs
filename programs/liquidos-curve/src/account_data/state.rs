use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct State {
  /// The owner that can handle various admin related tasks
  pub owner: Pubkey,
  /// The treasury account that receives fees
  pub treasury: Pubkey,
  /// Current protocol fees (fixed lamports amount). This is applied when the pool is created on Raydium
  pub protocol_fee: u64,
  /// Current trade fees (BPS). This is applied on each trade that takes place. Fees collected in SOL
  pub trade_fee_bps: u64,
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
  pub const MAX_SIZE: usize = 8 + Self::INIT_SPACE;

  pub fn new(
    owner: Pubkey,
    treasury: Pubkey,
    protocol_fee: u64,
    trade_fee_bps: u64,
    creator_fee: u64,
    total_token_supply: u64,
    staking_allocation: u64,
  ) -> Self {
    Self {
      owner,
      treasury,
      protocol_fee,
      trade_fee_bps,
      creator_fee,
      total_token_supply,
      staking_program: None,
      staking_program_state: None,
      staking_allocation,
    }
  }
}
