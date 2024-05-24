use std::str::FromStr;
use anchor_lang::prelude::*;
use anchor_spl::{
  token::Token, token_interface::{TokenInterface, Mint, TokenAccount},
  associated_token::AssociatedToken,
};
use crate::{
  account_data::{bonding_curve::BondingCurve, state::State}, raydium,
  program_error::ErrorCode,
};

#[derive(Accounts)]
pub struct MoveLiquidity<'info> {
  #[account(mut)]
  pub buyer: Signer<'info>,

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

  /// The ATA of the  token that is owned by the buyer. Create one if no already exists
  #[account(
    mut,
    associated_token::mint = token,
    associated_token::authority = buyer,
    associated_token::token_program = token_2022,
  )]
  pub buyer_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  /// The ATA of the WSOL token that is owned by the buyer. Create one if no already exists
  #[account(
    mut,
    associated_token::mint = wsol_token,
    associated_token::authority = buyer,
  )]
  pub buyer_wsol_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  // ---------------- Raydium CP swap accounts ----------------
  
  /// CHECK: Which config the pool that will created belongs to. Checks will take place in CP swap program
  #[account(
    address = raydium::amm_config(),
  )]
  pub amm_config: AccountInfo<'info>,

  /// CHECK: pool vault and lp mint authority. Checks will take place in CP swap program
  pub raydium_authority: AccountInfo<'info>,

  /// CHECK: Initialize an account to store the pool state. Checks will take place in CP swap program
  #[account(mut)]
  pub pool_state: AccountInfo<'info>,

  /// CHECK: the wsol token account
  #[account(
    constraint = wsol_token.key() == Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
  )]
  pub wsol_token: AccountInfo<'info>,

  /// CHECK: pool lp mint. Checks will take place in CP swap program
  #[account(mut)]
  pub lp_mint: AccountInfo<'info>,

  /// CHECK: creator lp token account
  #[account(mut)]
  pub creator_lp_token: AccountInfo<'info>,

  /// CHECK: Token_0 vault for the pool. Checks will take place in CP swap program
  #[account(mut)]
  pub token_0_vault: UncheckedAccount<'info>,
  /// CHECK: Token_0 vault for the pool. Checks will take place in CP swap program
  #[account(mut)]
  pub token_1_vault: UncheckedAccount<'info>,
  /// CHECK: create pool fee account. Checks will take place in CP swap program
  #[account(mut)]
  pub create_pool_fee: AccountInfo<'info>,
  /// CHECK: an account to store oracle observations. Checks will take place in CP swap program
  #[account(mut)]
  pub observation_state: AccountInfo<'info>,

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Program<'info, Token>,
  pub token_2022: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}

