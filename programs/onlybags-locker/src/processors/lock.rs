use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use anchor_spl::token_2022::{self, TransferChecked};
use crate::{
  program_error::ErrorCode,
  account_data::{token_lock::TokenLock, user_lock::UserLock},
  instructions::lock::Lock,
};

pub const TOKEN_DECIMALS: u8 = 6;

#[event]
pub struct LockEvent {
  user: Pubkey,
  amount: String,
  token: Pubkey,
  start_ts: i64,
  duration: i64,
  user_total_lock: String,
  token_total_lock: String,
}

fn lock_funds(ctx: &Context<Lock>, amount: u64) -> Result<()> {
  let cpi_accounts = TransferChecked {
    from: ctx.accounts.user_ata.to_account_info(),
    mint: ctx.accounts.token.to_account_info(),
    to: ctx.accounts.escrow_ata.to_account_info(),
    authority: ctx.accounts.user.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

  token_2022::transfer_checked(cpi_ctx, amount, TOKEN_DECIMALS)?;

  Ok(())
}

pub fn lock_expired(user_lock: &UserLock, now: i64) -> Result<bool> {
  Ok(now as u64 > (user_lock.start_ts as u64).safe_add(user_lock.duration as u64)?)
}

pub fn exec(
  ctx: Context<Lock>,
  amount: u64,
  duration: i64,
  _test_ts: i64,
) -> Result<()> {
  #[cfg(not(feature = "localnet"))]
  let now = Clock::get().unwrap().unix_timestamp;
  #[cfg(feature = "localnet")]
  let now = _test_ts;

  {
    // init both account only once
    let user_lock = &mut ctx.accounts.user_lock;
    if !user_lock.initialized {
      **user_lock = UserLock::new(now, duration, ctx.bumps.user_lock);
    }
    let token_lock = &mut ctx.accounts.token_lock;
    if !token_lock.initialized {
      **token_lock = TokenLock::new(ctx.bumps.token_lock);
    }
    
    // if lock expired and there is still some funds in the escrow then user must unlock that amount first before
    // creating a new lock
    if lock_expired(&user_lock, now)? && user_lock.total_locked > 0 {
      return Err(error!(ErrorCode::LockExpired))
    }

    // if expired an no amount is locked atm then simply create a new lock period
    if lock_expired(&user_lock, now)? {
      user_lock.start_ts = now;
      user_lock.duration = duration;
    }
  }

  lock_funds(&ctx, amount)?;

  // update state
  let token_lock = &mut ctx.accounts.token_lock;
  let user_lock = &mut ctx.accounts.user_lock;
  token_lock.total_locked = token_lock.total_locked.safe_add(amount)?;
  user_lock.total_locked = user_lock.total_locked.safe_add(amount)?;

  emit_cpi!(LockEvent {
    user: ctx.accounts.user.key(),
    amount: amount.to_string(),
    token: ctx.accounts.token.key(),
    start_ts: user_lock.start_ts,
    duration: user_lock.duration,
    user_total_lock: user_lock.total_locked.to_string(),
    token_total_lock: token_lock.total_locked.to_string(),
  });

  Ok(())
}
