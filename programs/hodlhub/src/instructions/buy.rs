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
    seeds = [b"bonding_curve", state.key().as_ref(), token.key().as_ref()],
    bump = bonding_curve.bump,
  )]
  pub bonding_curve: Account<'info, BondingCurve>,

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

  /// The ATA of the WSOL token that is owned by the buyer. Create one if no already exists
  #[account(
    init_if_needed,
    payer = buyer,
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
  pub pool_state: AccountInfo<'info>,

  #[account(
    constraint = wsol_token.key() == Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
  )]
  pub wsol_token: InterfaceAccount<'info, Mint>,

  /// CHECK: pool lp mint. Checks will take place in CP swap program
  pub lp_mint: AccountInfo<'info>,

  /// creator lp token account
  #[account(
    associated_token::mint = lp_mint,
    associated_token::authority = buyer,
    token::token_program = token_program,
  )]
  pub creator_lp_token: Box<InterfaceAccount<'info, TokenAccount>>,

  /// CHECK: Token_0 vault for the pool. Checks will take place in CP swap program
  pub token_0_vault: UncheckedAccount<'info>,
  /// CHECK: Token_0 vault for the pool. Checks will take place in CP swap program
  pub token_1_vault: UncheckedAccount<'info>,
  /// CHECK: create pool fee account. Checks will take place in CP swap program
  pub create_pool_fee: AccountInfo<'info>,
  /// CHECK: an account to store oracle observations. Checks will take place in CP swap program
  pub observation_state: AccountInfo<'info>,

  pub associated_token_program: Program<'info, AssociatedToken>,
  /// Program to create mint account and mint tokens
  pub token_program: Program<'info, Token>,
  pub token_2022: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}

