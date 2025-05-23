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

  /// CHECK: This is the authority that will control the ATA of each pool and execute CPIs
  #[account(
    init,
    space = 0,
    payer = owner,
    seeds = [b"pool_authority", state.key().as_ref()],
    bump,
  )]
  pub pool_authority: AccountInfo<'info>,
  
  #[account(mut)]
  pub owner: Signer<'info>,
  
  pub system_program: Program<'info, System>,
}
