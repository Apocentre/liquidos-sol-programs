use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use anchor_spl::token_2022::{self, TransferChecked};
use crate::{
  program_error::ErrorCode,
  instructions::unlock::UnLock,
};
use super::lock::{lock_expired, TOKEN_DECIMALS};

#[event]
pub struct UnlockEvent {
  user: Pubkey,
  amount: String,
  token: Pubkey,
  token_total_lock: String,
}

fn unlock_funds(ctx: &Context<UnLock>, amount: u64) -> Result<()> {
  let state_key = ctx.accounts.state.key();
  let seeds: &[&[u8]] = &[
    b"escrow",
    state_key.as_ref(),
    &[ctx.accounts.state.escrow_bump],
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];


  let cpi_accounts = TransferChecked {
    from: ctx.accounts.escrow_ata.to_account_info(),
    mint: ctx.accounts.token.to_account_info(),
    to: ctx.accounts.user_ata.to_account_info(),
    authority: ctx.accounts.escrow.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  token_2022::transfer_checked(cpi_ctx, amount, TOKEN_DECIMALS)?;

  Ok(())
}

pub fn exec(ctx: Context<UnLock>) -> Result<()> {
  let now = Clock::get().unwrap().unix_timestamp;
  
  let user_lock = &ctx.accounts.user_lock;
  require!(lock_expired(&user_lock, now)?, ErrorCode::LockNotExpired);

  if user_lock.total_locked > 0 {
    unlock_funds(&ctx, user_lock.total_locked)?;
  }

  // update state
  let token_lock = &mut ctx.accounts.token_lock;
  let user_lock = &mut ctx.accounts.user_lock;
  token_lock.total_locked = token_lock.total_locked.safe_sub(user_lock.total_locked)?;
  user_lock.total_locked = 0;

  emit_cpi!(UnlockEvent {
    user: ctx.accounts.user.key(),
    token: ctx.accounts.token.key(),
    amount: user_lock.total_locked.to_string(),
    token_total_lock: token_lock.total_locked.to_string(),
  });

  Ok(())
}
