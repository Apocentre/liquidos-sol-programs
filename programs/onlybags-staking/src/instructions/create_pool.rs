use anchor_lang::prelude::*;
use crate::account_data::state::State;

#[derive(Accounts)]
pub struct CreatePool<'info> {
  /// The state account of each instance of this program
  #[account()]
  pub state: Account<'info, State>,
  
  #[account(
    mut,
    seeds = [b"ticket_sale:cpi_authority", state.ticket_sale_state.as_ref()],
    bump = cpi_authority_bump,
    seeds::program = state.ticket_sale_program,
  )]
  pub onlybags_cpi_authority: Signer<'info>,
  
  pub system_program: Program<'info, System>,
}
