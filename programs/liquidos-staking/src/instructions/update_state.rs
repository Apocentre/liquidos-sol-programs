use anchor_lang::prelude::*;
use crate::{
  account_data::state::State, program_error::ErrorCode,
};

#[derive(Accounts)]
pub struct UpdateState<'info> {
  /// The state account of each instance of this program
  #[account(mut)]
  pub state: AccountLoader<'info, State>,

  #[account(
    constraint = owner.key() == state.load()?.owner @ ErrorCode::OnlyOwner,
  )]
  pub owner: Signer<'info>,
}
