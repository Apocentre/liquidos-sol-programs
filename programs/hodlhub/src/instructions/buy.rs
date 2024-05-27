use std::str::FromStr;
use anchor_lang::{prelude::*, solana_program::sysvar};
use anchor_spl::{
  token::Token, token_interface::{TokenInterface, Mint, TokenAccount},
  associated_token::AssociatedToken,
};
use crate::{
  account_data::{bonding_curve::BondingCurve, state::State},
  program_error::ErrorCode, raydium,
};

#[derive(Accounts)]
pub struct Buy<'info> {
  #[account(mut)]
  pub buyer: Signer<'info>,

  /// The state account of each instance of this program
  #[account()]
  pub state: Box<Account<'info, State>>,

  /// CHECK: The treasury account that collects the fees
  #[account(
    mut,
    constraint = treasury.key() == state.treasury @ ErrorCode::WrongTreasury,
  )]
  pub treasury: AccountInfo<'info>,

  /// The state of the bonding curve that will be used during buys and sells
  #[account(
    mut,
    seeds = [b"bonding_curve", state.key().as_ref(), token.key().as_ref()],
    bump = bonding_curve.bump,
  )]
  pub bonding_curve: Box<Account<'info, BondingCurve>>,

  /// The ATA of the WSOL token that is owned by the buyer. Create one if no already exists
  #[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = wsol_token,
    associated_token::authority = buyer,
  )]
  pub buyer_wsol_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  /// CHECK: the wsol token account
  #[account(
    constraint = wsol_token.key() == Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
  )]
  pub wsol_token: AccountInfo<'info>,
  
  #[account(
    mut,
    constraint = token.key() == bonding_curve.token @ ErrorCode::InvalidCurveToken,
  )]
  pub token: Box<InterfaceAccount<'info, Mint>>,

  /// CHECK: Which config the pool that will created belongs to. Checks will take place in CP swap program
  #[account(
    address = raydium::amm_config(),
  )]
  pub amm_config: AccountInfo<'info>,

  /// The ATA of the  token that is owned by the buyer. Create one if no already exists
  #[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = token,
    associated_token::authority = buyer,
    associated_token::token_program = token_2022,
  )]
  pub buyer_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_2022: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,
  /// CHECK: custom constrain to check the correctness of the account address
  #[account(address = sysvar::instructions::id())]
  pub ix_sysvar: UncheckedAccount<'info>,
}

