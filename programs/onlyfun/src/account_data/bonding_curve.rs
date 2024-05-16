
use std::mem::size_of;
use anchor_lang::prelude::*;

pub const MAX_OPERATORS: usize = 5;

#[account]
pub struct BondingCurve {}

impl BondingCurve {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();
}
