use anchor_lang::prelude::*;
use crate::{
  account_data::state::State, program_error::ErrorCode,
};

#[derive(Accounts)]
#[instruction(size: u64)]
pub struct ResizeState<'info> {
  #[account(
    mut,
    realloc = size as usize,
    realloc::payer = payer,
    realloc::zero = false,
  )]
  pub state: Account<'info, State>,

  #[account(
    mut,
    constraint = payer.key() == state.owner @ ErrorCode::OnlyOwner,
  )]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
}
