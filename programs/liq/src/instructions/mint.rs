use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken, token_interface::{Mint as SplMint, TokenInterface, TokenAccount},
};
use crate::{
  account_data::{bonding_curve::BondingCurve, state::State}, program_error::ErrorCode,
};

#[derive(Accounts)]
pub struct Mint<'info> {
  /// The state account of each instance of this program
  #[account()]
  pub state: Box<Account<'info, State>>,

  #[account(mut)]
  pub buyer: Signer<'info>,

  /// The state of the bonding curve that will be used during buys and sells
  #[account(
    mut,
    seeds = [b"bonding_curve", state.key().as_ref()],
    bump = bonding_curve.bump,
  )]
  pub bonding_curve: Box<Account<'info, BondingCurve>>,

  #[account(
    mut,
    constraint = liq_token.key() == bonding_curve.liq_token @ ErrorCode::InvalidLiqToken,
  )]
  pub liq_token: Box<InterfaceAccount<'info, SplMint>>,

  #[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = liq_token,
    associated_token::authority = buyer,
    associated_token::token_program = token_2022,
  )]
  pub buyer_liq_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  #[account(mut)]
  pub curve_creator: AccountInfo<'info>,

  #[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = liq_token,
    associated_token::authority = curve_creator,
    associated_token::token_program = token_2022,
  )]
  pub curve_creator_liq_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  pub token_2022: Interface<'info, TokenInterface>,
  associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
}
