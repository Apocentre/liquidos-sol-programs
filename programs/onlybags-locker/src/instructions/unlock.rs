use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  token_interface::{Mint, TokenAccount, TokenInterface}
};
use crate::account_data::{state::State, token_lock::TokenLock, user_lock::UserLock};

#[derive(Accounts)]
pub struct UnLock<'info> {
  #[account()]
  pub state: Box<Account<'info, State>>,

  #[account(
    mut,
    seeds = [b"token_lock", state.key().as_ref(), token.key().as_ref()],
    bump,
  )]
  pub token_lock: Account<'info, TokenLock>,
  
  #[account(
    mut,
    seeds = [b"user_lock", state.key().as_ref(), token.key().as_ref(), user.key().as_ref()],
    bump,
  )]
  pub user_lock: Account<'info, UserLock>,

  #[account()]
  pub token: Box<InterfaceAccount<'info, Mint>>,

  /// ATA that will store the reward tokens
  #[account(
    mut,
    associated_token::mint = token,
    associated_token::authority = user,
    associated_token::token_program = token_2022,
  )]
  pub user_ata: Box<InterfaceAccount<'info, TokenAccount>>,
  
  /// CHECK: This is the authority of all the ATA that will store the staked tokens
  #[account(
    mut,
    seeds = [b"escrow", state.key().as_ref()],
    bump = state.escrow_bump,
  )]
  pub escrow: AccountInfo<'info>,
    
  #[account(
    init_if_needed,
    payer = user,
    associated_token::mint = token,
    associated_token::authority = escrow,
    associated_token::token_program = token_2022,
  )]
  pub escrow_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  #[account(mut)]
  pub user: Signer<'info>,
  
  pub token_2022: Interface<'info, TokenInterface>,
  associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
}
