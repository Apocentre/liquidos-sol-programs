use anchor_lang::prelude::*;
use crate::account_data::state::State;

#[derive(Accounts)]
pub struct Initialize<'info> {
  /// The state account of each instance of this program
  #[account(
    init,
    payer = owner,
    space = State::MAX_SIZE,
  )]
  pub state: Account<'info, State>,
  
  /// CHECK: This is the authority that will control the escrow ATA 
  #[account(
    init,
    space = 0,
    payer = owner,
    seeds = [b"escrow", state.key().as_ref()],
    bump,
  )]
  pub escrow: AccountInfo<'info>,

  #[account(mut)]
  pub owner: Signer<'info>,
  
  pub system_program: Program<'info, System>,
}
