use anchor_lang::prelude::*;
use anchor_spl::{
  token::Token, token_interface::{TokenInterface, TokenAccount},
  associated_token::AssociatedToken,
};
use crate::{
  account_data::state::State, raydium,
};

#[derive(Accounts)]
pub struct Swap<'info> {
  #[account(mut)]
  pub user: Signer<'info>,

  /// The state account of each instance of this program
  #[account()]
  pub state: Box<Account<'info, State>>,

  /// The treasury input mint ata
  #[account(
    init_if_needed,
    payer = user,
    associated_token::mint = input_token_mint,
    associated_token::authority = user,
  )]
  pub treasury_input_ata: Box<InterfaceAccount<'info, TokenAccount>>,
  
  /// The treasury output mint ata
  #[account(
    init_if_needed,
    payer = user,
    associated_token::mint = output_token_mint,
    associated_token::authority = user,
  )]
  pub treasury_output_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    
  // ---------------- Raydium CP swap accounts ----------------
  
  /// CHECK: pool vault and lp mint authority. Checks will take place in CP swap program
  pub raydium_authority: AccountInfo<'info>,

  /// CHECK: Which config the pool that will created belongs to. Checks will take place in CP swap program
  #[account(
    address = raydium::amm_config(),
  )]
  pub amm_config: AccountInfo<'info>,

  /// CHECK: Initialize an account to store the pool state. Checks will take place in CP swap program
  #[account(mut)]
  pub pool_state: AccountInfo<'info>,

  /// CHECK: The user token account for input token. Checks will take place in CP swap program
  #[account(mut)]
  pub input_token_account: AccountInfo<'info>,

  /// CHECK: The user token account for output token. Checks will take place in CP swap program
  #[account(mut)]
  pub output_token_account: AccountInfo<'info>,

  /// CHECK: The vault token account for input token. Checks will take place in CP swap program
  #[account(mut)]
  pub input_vault: AccountInfo<'info>,

  /// CHECK: The vault token account for input token. Checks will take place in CP swap program
  #[account(mut)]
  pub output_vault: AccountInfo<'info>,

  /// CHECK: The mint of input token. Checks will take place in CP swap program
  #[account(mut)]
  pub input_token_mint: AccountInfo<'info>,

  /// CHECK: The mint of output token. Checks will take place in CP swap program
  #[account(mut)]
  pub output_token_mint: AccountInfo<'info>,

  /// CHECK: an account to store oracle observations. Checks will take place in CP swap program
  #[account(mut)]
  pub observation_state: AccountInfo<'info>,

  /// CHECK: Raydium CP swap program
  #[account(address = raydium::id())]
  pub cp_swap_program: AccountInfo<'info>,

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Program<'info, Token>,
  pub token_2022: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,
}
