
use std::mem::size_of;
use anchor_lang::prelude::*;
use math::utils::{BPS, calc_perc_value};
use crate::program_error::ErrorCode;

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Treasury {
  pub acc: Pubkey,
  pub fee_bps: u64,
}

#[account]
pub struct State {
  /// The owner that can handle various admin related tasks
  pub owner: Pubkey,
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
  /// The treasury accounts that receives fees and the corresponding portion each received from the trade_fee
  /// NOTE! the first treasury is always our platform treasury wallet
  pub treasuries: Vec<Treasury>,
}

impl State {
  pub const MAX_TREASURY_ACCOUNTS: usize = 5;
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>()
  + Self::MAX_TREASURY_ACCOUNTS * size_of::<Treasury>();

  pub fn new(
    owner: Pubkey,
    treasuries: Vec<Treasury>,
    protocol_fee: u64,
    trade_fee_bps: u64,
    creator_fee: u64,
    total_token_supply: u64,
    staking_allocation: u64,
  ) -> Result<Self> {
    let total_trade_fees: u64 = treasuries.iter().map(|t| t.fee_bps).sum();
    require!(total_trade_fees == BPS, ErrorCode::TradeFeesMisconfiguration);

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
    let Some(t) = self.treasuries.iter().find(|t| t.acc.eq(treasury)) else {
      return Err(ErrorCode::WrongTreasury.into())
    };

    Ok(
      calc_perc_value(total_fees, t.fee_bps)?
    )
  }
}
