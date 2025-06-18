use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken, token_interface::{Mint, TokenInterface},
};
use crate::{account_data::{bonding_curve::BondingCurve, state::State}, program_error::ErrorCode};
use liq::program::Liq;

#[derive(Accounts)]
#[instruction(curve_token: Pubkey)]
pub struct MintLiq<'info> {
  /// The state account of each instance of this program
  #[account()]
  pub state: Box<Account<'info, State>>,

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

  /// The state account of each instance of this program
  #[account(
    seeds = [b"liq_state"],
    bump,
    seeds::program = Liq::id(),
  )]
  pub liq_state: AccountInfo<'info>,

  #[account(mut)]
  pub buyer: Signer<'info>,

  /// CHECK: The state of the bonding curve that will be used during buys and sells
  #[account(
    mut,
    seeds = [b"liq_bonding_curve", liq_state.key().as_ref()],
    bump,
    seeds::program = Liq::id(),
  )]
  pub liq_bonding_curve: AccountInfo<'info>,

  /// CHECK: The liq token created in the Liq program
  #[account(mut)]
  pub liq_token: AccountInfo<'info>,

  /// CHECK: The liq token created in the Liq program
  #[account(mut)]
  pub buyer_liq_ata: AccountInfo<'info>,

  /// CHECK: The curve creator
  #[account(mut)]
  pub curve_creator: AccountInfo<'info>,

  /// CHECK: The liq token owned by the curve token creator
  #[account(mut)]
  pub curve_creator_liq_ata: AccountInfo<'info>,

  /// CHECK: This is the Liq program account
  #[account(
    address = Liq::id() @ ErrorCode::WrongLiqProgram,
  )]
  pub liq_program: Program<'info, Liq>,

  pub token_2022: Interface<'info, TokenInterface>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
}
