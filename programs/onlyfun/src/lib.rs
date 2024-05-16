pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;

use anchor_lang::prelude::*;
use crate::instructions::initialize::*;

declare_id!("2d6f7qg9SnGaLSN1EejmD3da72bJppqmKnB6C21zFNHj");

#[program]
pub mod onlyfun {
  use super::*;

  /// Initialize
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `operators` - The list of all operators that can run admin related tasks
  /// * `current_sol_target` - Current target of SOL each pool should receive before it goes to the 
  pub fn initialize(
    ctx: Context<Initialize>,
    operators: Vec<Pubkey>,
    current_sol_target: u64,
  ) -> Result<()> {
    processors::initialize::exec(
      ctx,
      operators,
      current_sol_target,
    )
  }
}
