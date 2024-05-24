use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken, token_2022::spl_token_2022::extension::ExtensionType,
  token_interface::{Mint, TokenInterface, TokenAccount},
};
use crate::account_data::{bonding_curve::BondingCurve, state::State};

pub const MINT_EXTENSIONS: [ExtensionType; 1] = [ExtensionType::MetadataPointer];

#[derive(Accounts)]
pub struct CreateToken<'info> {
  /// The state account of each instance of this program
  #[account()]
  pub state: Box<Account<'info, State>>,

  /// The Mint account of the newly created token.
  #[account(
    init,
    payer = token_creator,
    mint::decimals = 6,
    mint::authority = bonding_curve,
    mint::token_program = token_2022,
    // mint::extensions = MINT_EXTENSIONS.to_vec(),
    extensions::metadata_pointer::authority = bonding_curve.key(),
    extensions::metadata_pointer::metadata_address = token.key(),
  )]
  pub token: Box<InterfaceAccount<'info, Mint>>,

  /// The ATA that will hold the liquidity of the curve (token side).
  #[account(
    init,
    payer = token_creator,
    associated_token::mint = token,
    associated_token::authority = bonding_curve,
    associated_token::token_program = token_2022,
  )]
  pub curve_ata: Box<InterfaceAccount<'info, TokenAccount>>,

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
}
