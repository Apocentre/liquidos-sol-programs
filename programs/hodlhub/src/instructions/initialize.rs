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

  /// CHECK: The PDA that will be sending CPI to other programs i.e. Raydium DEX Program
  #[account(
    init,
    payer = owner,
    space = 0,
    seeds = [b"cpi_authority", state.key().as_ref()],
    bump,
  )]
  pub cpi_authority: AccountInfo<'info>,
  
  #[account(mut)]
  pub owner: Signer<'info>,
  
  pub system_program: Program<'info, System>,
}
