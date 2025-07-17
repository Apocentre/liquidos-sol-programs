use std::str::FromStr;

use anchor_lang::prelude::*;
use crate::{
  account_data::{migration::Migration, state::State, bonding_curve::BondingCurve}, program_error::ErrorCode,
};

#[derive(Accounts)]
#[instruction(size: u64)]
pub struct ResizeBondingCurve<'info> {
  /// The state account of each instance of this program
  #[account()]
  pub state: Account<'info, Migration::<State>>,

  #[account(
    mut,
    realloc = size as usize,
    realloc::payer = payer,
    realloc::zero = false,
  )]
  pub bonding_curve: Account<'info, Migration::<BondingCurve>>,

  #[account(
    mut,
    constraint = payer.key() == Pubkey::from_str("DxVMyJ9YGahVLDXwEb5RaWcFx89JcAErCYGTJrPrneiw").unwrap() @ ErrorCode::OnlyOwner,
  )]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
}
