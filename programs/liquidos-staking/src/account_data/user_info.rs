use anchor_lang::prelude::*;

use crate::constants::SPACE_MARGIN;

#[account]
#[derive(InitSpace)]
pub struct UserInfo {
  pub staked_amount: u64,
  pub reward_debt: u64,
  pub total_claimed: u64,
  pub bump: u8,
}

impl UserInfo {
  pub const MAX_SIZE: usize = 8 + Self::INIT_SPACE + SPACE_MARGIN;
}
