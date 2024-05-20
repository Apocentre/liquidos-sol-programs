use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::{
  token_interface::{TokenInterface, Mint, TokenAccount},
  associated_token::AssociatedToken,
};
use crate::{account_data::{bonding_curve::BondingCurve, state::State}, raydium::constants::{AUTH_SEED, POOL_SEED}};

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

  /// CHECK: The PDA is the authority of the token. This account can mint and burn tokens
  #[account(
    seeds = [b"token_authority", state.key().as_ref(), token.key().as_ref()],
    bump = bonding_curve.token_authority_bump,
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

  /// The ATA that will hold the liquidity of the curve (token side)
  #[account(
    associated_token::mint = token,
    associated_token::authority = token_authority,
    associated_token::token_program = token_2022,
  )]
  pub curve_ata: Box<InterfaceAccount<'info, TokenAccount>>,
  
  #[account(mut)]
  pub buyer: Signer<'info>,

  // ---------------- Raydium CP swap accounts ----------------
  
  /// CHECK: Which config the pool that will created belongs to. Checks will take place in CP swap program
  pub amm_config: AccountInfo<'info>,

  /// CHECK: pool vault and lp mint authority
  #[account(
    seeds = [
      AUTH_SEED.as_bytes(),
    ],
    bump,
  )]
  pub authority: UncheckedAccount<'info>,

  /// CHECK: Initialize an account to store the pool state
  #[account(
    seeds = [
        POOL_SEED.as_bytes(),
        amm_config.key().as_ref(),
        // The order is important as for raydium token_0 mint, the key must smaller then token_1 mint.
        if token.key() < wsol_token.key() {token.key().clone()} else {wsol_token.key()}.as_ref(),
        if token.key() < wsol_token.key() {wsol_token.key()} else {token.key()}.as_ref(),
    ],
    bump,
  )]
  pub pool_state: AccountInfo<'info>,

  #[account(
    address = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
  )]
  pub wsol_token: Box<InterfaceAccount<'info, Mint>>,

  /// pool lp mint
  // #[account(
  //   seeds = [
  //     raydium_cp_swap::states::pool::POOL_LP_MINT_SEED.as_bytes(),
  //     pool_state.key().as_ref(),
  //   ],
  //   bump,
  //   mint::authority = authority,
  //   mint::token_program = token_2022,
  // )]
  // pub lp_mint: Box<AccountInfo<'info>>,

  associated_token_program: Program<'info, AssociatedToken>,
  pub token_2022: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}
