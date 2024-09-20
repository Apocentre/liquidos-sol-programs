use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use anchor_spl::token_2022::{self, TransferChecked};
use crate::{
  account_data::{user_lock::UserLock, token_lock::TokenLock},
  instructions::lock::Lock,
};

pub const TOKEN_DECIMALS: u8 = 6;

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
  }

  lock_funds(&ctx, amount)?;

  // update state
  let token_lock = &mut ctx.accounts.token_lock;
  let user_lock = &mut ctx.accounts.user_lock;
  token_lock.total_locked = token_lock.total_locked.safe_add(amount)?;
  user_lock.total_locked = user_lock.total_locked.safe_add(amount)?;

  Ok(())
}
