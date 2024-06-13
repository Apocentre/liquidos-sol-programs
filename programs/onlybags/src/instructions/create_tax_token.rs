use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken, token_interface::TokenInterface,
};
use crate::account_data::{bonding_curve::BondingCurve, state::State};

#[derive(Accounts)]
#[instruction(name: String, symbol: String)]
pub struct CreateTaxToken<'info> {
  /// The state account of each instance of this program
  #[account()]
  pub state: Box<Account<'info, State>>,

  /// CHECK: The Mint account of the newly created token. The initialization and extension setup will
  /// take place in the processor function.
  #[account(
    mut,
    seeds = [b"onlybags_token", state.key().as_ref(), format!("{}-{}", name, symbol).as_ref()],
    bump,
  )]
  pub token: AccountInfo<'info>,

  /// CHECK: The ATA that will hold the liquidity of the curve (token side). The account will be initialized
  /// in the processor function
  #[account(mut)]
  pub curve_ata: AccountInfo<'info>,

  /// The state of the bonding curve that will be used during buys and sells
  #[account(
    init,
    payer = token_creator,
    space = BondingCurve::MAX_SIZE,
    seeds = [b"bonding_curve", state.key().as_ref(), token.key().as_ref()],
    bump,
  )]
  pub bonding_curve: Box<Account<'info, BondingCurve>>,

  /// The user that is creating the token
  #[account(mut)]
  pub token_creator: Signer<'info>,

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_2022: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
