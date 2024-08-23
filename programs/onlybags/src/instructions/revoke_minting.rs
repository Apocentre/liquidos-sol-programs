use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenInterface, Mint};
use crate::{
  account_data::{bonding_curve::BondingCurve, state::State},
  program_error::ErrorCode,
};

#[derive(Accounts)]
pub struct RevokeMinting<'info> {
  /// The state account of each instance of this program
  #[account()]
  pub state: Box<Account<'info, State>>,

  /// The state of the bonding curve that will be used during buys and sells
  #[account(
    seeds = [b"bonding_curve", state.key().as_ref(), token.key().as_ref()],
    bump = bonding_curve.bump,
  )]
  pub bonding_curve: Box<Account<'info, BondingCurve>>,

  #[account(
    constraint = token.key() == bonding_curve.token @ ErrorCode::InvalidCurveToken,
  )]
  pub token: Box<InterfaceAccount<'info, Mint>>,

  pub token_2022: Interface<'info, TokenInterface>,
}

