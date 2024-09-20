use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use anchor_spl::token_2022::{self, TransferChecked};
use crate::{
  program_error::ErrorCode,
  account_data::user_lock::UserLock,
  instructions::unlock::UnLock,
};

pub const TOKEN_DECIMALS: u8 = 6;

#[event]
pub struct UnLockEvent {
  user: Pubkey,
  amount: u64,
  token: Pubkey,
  token_total_lock: u64,
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

fn lock_expired(user_lock: &UserLock, now: i64) -> Result<bool> {
  Ok(now as u64 > (user_lock.start_ts as u64).safe_add(user_lock.duration as u64)?)
}

pub fn exec(ctx: Context<UnLock>, _test_ts: i64) -> Result<()> {
  #[cfg(not(feature = "localnet"))]
  let now = Clock::get().unwrap().unix_timestamp;
  #[cfg(feature = "localnet")]
  let now = _test_ts;
  
  let user_lock = &ctx.accounts.user_lock;
  require!(lock_expired(&user_lock, now)?, ErrorCode::LockNotExpired);

  if user_lock.total_locked > 0 {
    unlock_funds(&ctx, user_lock.total_locked)?;
  }

  // update state
  let token_lock = &mut ctx.accounts.token_lock;
  let user_lock = &mut ctx.accounts.user_lock;
  user_lock.total_locked = 0;
  token_lock.total_locked = token_lock.total_locked.safe_sub(user_lock.total_locked)?;

  emit!(UnLockEvent {
    user: ctx.accounts.user.key(),
    token: ctx.accounts.token.key(),
    amount: user_lock.total_locked,
    token_total_lock: token_lock.total_locked,
  });

  Ok(())
}
