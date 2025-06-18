use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken, token_interface::{Mint, TokenInterface},
};
use crate::{
  account_data::{bonding_curve::BondingCurve, state::State},
  constants::allowed_deployer, program_error::ErrorCode,
};

#[derive(Accounts)]
#[instruction(name: String, symbol: String)]
pub struct Initialize<'info> {
  /// The state account of each instance of this program
  #[account(
    init,
    payer = deployer,
    space = State::MAX_SIZE,
  )]
  pub state: Account<'info, State>,

  /// The state of the bonding curve that will be used during buys and sells
  #[account(
    init,
    payer = deployer,
    space = BondingCurve::MAX_SIZE,
    seeds = [b"bonding_curve", state.key().as_ref()],
    bump,
  )]
  pub bonding_curve: Box<Account<'info, BondingCurve>>,

  /// The Mint account of the newly created token.
  #[account(
    init,
    payer = deployer,
    mint::decimals = 6,
    mint::authority = bonding_curve,
    mint::token_program = token_2022,
    extensions::metadata_pointer::authority = bonding_curve.key(),
    extensions::metadata_pointer::metadata_address = liq_token.key(),
    seeds = [b"liq_token", state.key().as_ref(), format!("{}-{}", name, symbol).as_ref()],
    bump,
  )]
  pub liq_token: Box<InterfaceAccount<'info, Mint>>,

  #[account(
    mut,
    address = allowed_deployer() @ ErrorCode::WrongDeployer,
  )]
  pub deployer: Signer<'info>,
  
  pub token_2022: Interface<'info, TokenInterface>,
  associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
}
