
use std::mem::size_of;
use anchor_lang::prelude::*;

pub const MAX_OPERATORS: usize = 5;

#[account]
pub struct State {
  /// The owner that can handle various admin related teasks
  pub owner: Pubkey,
  /// The list of all operators that can run admin related tasks
  pub operators: Vec<Pubkey>,
}

impl State {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>()
  + size_of::<Pubkey>() * MAX_OPERATORS;
}
