
use std::mem::size_of;
use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use crate::program_error::ErrorCode;

#[account]
pub struct State {
  /// The owner that can handle various admin related tasks
  pub owner: Pubkey,
  /// The treasury accounts that receives fees and the corresponding portion each received from the trade_fee
  pub treasuries: Vec<(Pubkey, u64)>,
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
  pub const MAX_TREASURY_ACCOUNTS: usize = 5;
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>()
  + Self::MAX_TREASURY_ACCOUNTS * size_of::<Pubkey>();

  pub fn new(
    owner: Pubkey,
    treasuries: Vec<(Pubkey, u64)>,
    protocol_fee: u64,
    trade_fee_bps: u64,
    creator_fee: u64,
    total_token_supply: u64,
    staking_allocation: u64,
  ) -> Result<Self> {
    let total_trade_fees: u64 = treasuries.iter().map(|(_, t)| t).sum();
    require!(total_trade_fees == 10_000, ErrorCode::TradeFeesMisconfiguration);

    Ok(Self {
      owner,
      treasuries,
      protocol_fee,
      trade_fee_bps,
      creator_fee,
      total_token_supply,
      staking_program: None,
      staking_program_state: None,
      staking_allocation,
    })
  }

  pub fn calc_treasury_fee(&self, treasury: &Pubkey, total_fees: u64) -> Result<u64> {
    let Some((_, fee)) = self.treasuries.iter().find(|(t, _)| t.eq(treasury)) else {
      return Err(ErrorCode::WrongTreasury.into())
    };

    Ok(
      total_fees.safe_mul(*fee)?.safe_div(10_000)?
    )
  }
}
