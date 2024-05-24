use anchor_lang::{prelude::*, solana_program::program_pack::Pack};
use anchor_spl::{
  associated_token::AssociatedToken, token_2022::spl_token_2022,
  token_interface::{TokenAccount, TokenInterface}
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
  #[account(
    init,
    payer = token_creator,
    space = spl_token_2022::state::Mint::LEN,
  )]
  pub token: AccountInfo<'info>,

  /// The ATA that will hold the liquidity of the curve (token side).
  /// Since we're initializing token mint account manually, we need to do so here as well because
  /// creating an ata requires that mint token is initialized.
  #[account(
    init,
    payer = token_creator,
    space = spl_token_2022::state::Account::LEN,
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
