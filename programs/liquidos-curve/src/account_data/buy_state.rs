
use anchor_lang::prelude::*;

const SPACE_MARGIN: usize = 100;

#[account]
#[derive(InitSpace, Debug)]
pub struct BuyState {
  pub buy_amount: u64,
}

impl BuyState {
  pub const MAX_SIZE: usize = 8 + Self::INIT_SPACE + SPACE_MARGIN;
}
