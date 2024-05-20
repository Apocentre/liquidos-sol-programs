use anchor_lang::prelude::*;
use anchor_spl::{
  token_interface::{TokenInterface, Mint},
  associated_token::AssociatedToken,
};
use crate::account_data::{bonding_curve::BondingCurve, state::State};

#[derive(Accounts)]
pub struct CreateToken<'info> {
  /// The state account of each instance of this program
  #[account()]
  pub state: Box<Account<'info, State>>,

  /// The Mint account of the newly created token
  #[account(
    init,
    payer = token_creator,
    mint::decimals = 9,
    mint::authority = token_authority,
    mint::token_program = token_2022,
  )]
  pub token: InterfaceAccount<'info, Mint>,

  /// CHECK: The PDA is the authority of the newly created token. This account can mint and burn tokens
  #[account(
    init,
    payer = token_creator,
    space = 0,
    seeds = [b"token_authority", state.key().as_ref(), token.key().as_ref()],
    bump,
  )]
  pub token_authority: AccountInfo<'info>,

  /// The state of the bonding curve that will be used during buys and sells
  #[account(
    init,
    payer = token_creator,
    space = BondingCurve::MAX_SIZE,
    seeds = [b"bonding_curve", state.key().as_ref(), token.key().as_ref()],
    bump,
  )]
  pub bonding_curve: Account<'info, BondingCurve>,

  /// The user that is creating the token
  #[account(mut)]
  pub token_creator: Signer<'info>,

  associated_token_program: Program<'info, AssociatedToken>,
  pub token_2022: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,
}
