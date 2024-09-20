use anchor_lang::prelude::*;
use crate::account_data::{user_lock::UserLock, token_lock::TokenLock};

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(
    init_if_needed,
    payer = user,
    space = TokenLock::MAX_SIZE,
  )]
  pub token_lock: Account<'info, TokenLock>,
  
  #[account(
    init_if_needed,
    payer = user,
    space = UserLock::MAX_SIZE,
  )]
  pub user_lock: Account<'info, UserLock>,

  #[account(mut)]
  pub user: Signer<'info>,
  
  pub system_program: Program<'info, System>,
}
