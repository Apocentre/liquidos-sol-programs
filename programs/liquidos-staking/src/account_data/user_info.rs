use std::mem::size_of;

use anchor_lang::prelude::*;

#[account]
pub struct UserInfo {
  pub bump: u8,
  pub staked_amount: u64,
  pub reward_debt: u64,
  pub acc_claim: u64,
  pub total_claimed: u64,
  pub initialized: bool,
}

impl UserInfo {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();
}
