use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken, token_interface::{Mint, TokenInterface},
};
use crate::{
  account_data::state::State,
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

  /// The Mint account of the newly created token.
  #[account(
    init,
    payer = deployer,
    mint::decimals = 6,
    mint::authority = state,
    mint::token_program = token_2022,
    extensions::metadata_pointer::authority = state.key(),
    extensions::metadata_pointer::metadata_address = token.key(),
    seeds = [b"liq_token", state.key().as_ref(), format!("{}-{}", name, symbol).as_ref()],
    bump,
  )]
  pub token: Box<InterfaceAccount<'info, Mint>>,

  #[account(
    mut,
    address = allowed_deployer() @ ErrorCode::WrongDeployer,
  )]
  pub deployer: Signer<'info>,
  
  pub token_2022: Interface<'info, TokenInterface>,
  associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
}
