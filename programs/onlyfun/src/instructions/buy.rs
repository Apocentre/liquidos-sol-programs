use anchor_lang::prelude::*;
use anchor_spl::{
  token_interface::{TokenInterface, Mint, TokenAccount},
  associated_token::AssociatedToken,
};
use crate::account_data::{bonding_curve::BondingCurve, state::State};

#[derive(Accounts)]
pub struct Buy<'info> {
  /// The state account of each instance of this program
  #[account()]
  pub state: Account<'info, State>,

  /// The state of the bonding curve that will be used during buys and sells
  #[account(
    seeds = [b"bonding_curve", state.key().as_ref(), token.key().as_ref()],
    bump = bonding_curve.bump,
  )]
  pub bonding_curve: Account<'info, BondingCurve>,

  /// CHECK: The PDA is the authority of the newly created token. This account can mint and burn tokens
  #[account(
    seeds = [b"token_authority", state.key().as_ref(), token.key().as_ref()],
    bump = curve.token_authority_bump,
  )]
  pub token_authority: AccountInfo<'info>,


  #[account(mut)]
  pub token: Box<InterfaceAccount<'info, Mint>>,

  /// The ATA of the  token that is owned by the buyer. Create one if no already exists
  #[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = token,
    associated_token::authority = buyer,
    associated_token::token_program = token_2022,
  )]
  pub buyer_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  /// The ATA of the  token that is owned by the buyer. Create one if no already exists
  #[account(
    associated_token::mint = token,
    associated_token::authority = token_authority,
    associated_token::token_program = token_2022,
  )]
  pub curve_ata: Box<InterfaceAccount<'info, TokenAccount>>,
  
  #[account(mut)]
  pub buyer: Signer<'info>,

  associated_token_program: Program<'info, AssociatedToken>,
  pub token_2022: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,
}
