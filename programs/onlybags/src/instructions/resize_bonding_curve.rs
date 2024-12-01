use anchor_lang::prelude::*;
use crate::{
  account_data::{state::State, bonding_curve::BondingCurve}, program_error::ErrorCode,
};

#[derive(Accounts)]
#[instruction(size: u64)]
pub struct ResizeBondingCurve<'info> {
  /// The state account of each instance of this program
  #[account()]
  pub state: Box<Account<'info, State>>,

  #[account(
    mut,
    realloc = size as usize,
    realloc::payer = payer,
    realloc::zero = false,
  )]
  pub bonding_curve: Box<Account<'info, BondingCurve>>,

  #[account(
    mut,
    constraint = payer.key() == state.owner @ ErrorCode::OnlyOwner,
  )]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
}
