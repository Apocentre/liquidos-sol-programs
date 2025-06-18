use anchor_lang::prelude::*;
use crate::constants::SPACE_MARGIN;

#[account]
#[derive(InitSpace)]
pub struct State {
  /// The owner that can handle various admin related tasks
  pub owner: Pubkey,
  /// The address of the main liquidos curve program that will be CPIing into this program
  liquidos_curve_program: Pubkey
}

impl State {
  pub const MAX_SIZE: usize = 8 + Self::INIT_SPACE + SPACE_MARGIN;

  pub fn new(owner: Pubkey, liquidos_curve_program: Pubkey) -> Self {
    Self {
      owner,
      liquidos_curve_program,
    }
  }
}
