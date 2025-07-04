use anchor_lang::prelude::*;
use anchor_spl::{
  token_interface::{TokenInterface, Mint, TokenAccount},
  associated_token::AssociatedToken,
};
use crate::{
  account_data::{bonding_curve::BondingCurve, state::State},
  program_error::ErrorCode,
};

#[derive(Accounts)]
#[event_cpi]
pub struct Sell<'info> {
  #[account(mut)]
  pub seller: Signer<'info>,

  /// The state account of each instance of this program
  #[account()]
  pub state: Account<'info, State>,

  /// CHECK: The treasury account that collects the fees
  #[account(
    mut,
    constraint = treasury.key() == state.treasury @ ErrorCode::WrongTreasury,
  )]
  pub treasury: AccountInfo<'info>,

  /// The state of the bonding curve that will be used during buys and sells
  #[account(
    mut,
    seeds = [b"bonding_curve", state.key().as_ref(), token.key().as_ref()],
    bump = bonding_curve.bump,
  )]
  pub bonding_curve: Box<Account<'info, BondingCurve>>,

  #[account(
    mut,
    constraint = token.key() == bonding_curve.token @ ErrorCode::InvalidCurveToken,
  )]
  pub token: Box<InterfaceAccount<'info, Mint>>,

  /// The ATA of the  token that is owned by the seller. Create one if no already exists
  #[account(
    mut,
    associated_token::mint = token,
    associated_token::token_program = token_2022,
    associated_token::authority = seller,
  )]
  pub seller_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_2022: Interface<'info, TokenInterface>,
}
