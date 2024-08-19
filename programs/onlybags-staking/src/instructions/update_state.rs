use anchor_lang::prelude::*;
use crate::{
  account_data::state::State, program_error::ErrorCode,
};

#[derive(Accounts)]
pub struct UpdateState<'info> {
  /// The state account of each instance of this program
  #[account(mut)]
  pub state: Account<'info, State>,

  #[account(
    address = state.owner @ ErrorCode::OnlyOwner,
  )]
  pub owner: Signer<'info>,
}
