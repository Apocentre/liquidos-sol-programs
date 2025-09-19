
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
  /// Current trade fees i.e. fees collected on each swap
  pub trade_fee_bps: u64,
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
    trade_fee_bps: u64,
    treasuries: Vec<Treasury>,
  ) -> Result<Self> {
    let total_trade_fees: u64 = treasuries.iter().map(|t| t.fee_bps).sum();
    require!(total_trade_fees == BPS, ErrorCode::TradeFeesMisconfiguration);

    Ok(Self {
      owner,
      trade_fee_bps,
      treasuries,
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
