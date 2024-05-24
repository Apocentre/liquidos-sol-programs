use std::mem::size_of;
use anchor_lang::{prelude::*, solana_program::program_pack::Pack};
use anchor_spl::{
  associated_token::AssociatedToken, token_2022::spl_token_2022::{self, extension::metadata_pointer::MetadataPointer},
  token_interface::{spl_token_metadata_interface::state::TokenMetadata, TokenInterface},
};
use crate::account_data::{bonding_curve::BondingCurve, state::State};

#[derive(Accounts)]
pub struct CreateToken<'info> {
  /// The state account of each instance of this program
  #[account()]
  pub state: Box<Account<'info, State>>,

  /// CHECK: The Mint account of the newly created token. This will be manually initialized in the processor
  /// We do that in the processor because the order in which metadata pointer account is created is vital. It must
  /// be created before the Mint account is initialized.
  /// In the space we include the addiitonal space needed for the MetadataPointer data.
  #[account(
    init,
    payer = token_creator,
    space = spl_token_2022::state::Mint::LEN
      + size_of::<MetadataPointer>()
      + size_of::<TokenMetadata>(),
    owner = token_2022.key(),
  )]
  pub token: AccountInfo<'info>,

  /// CHECK: The ATA that will hold the liquidity of the curve (token side).
  /// Since we're initializing token mint account manually, we need to do so here as well because
  /// creating an ata requires that mint token is initialized.
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
}
