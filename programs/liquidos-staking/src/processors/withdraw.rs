use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use anchor_spl::token_2022::{self, TransferChecked};
use crate::{
  instructions::withdraw::Withdraw, program_error::ErrorCode,
  staking::{release_pending, update_pool, AccountContainer, NORMALIZATION_FACTOR, TOKEN_DECIMALS},
};

#[event]
pub struct WithdrawEvent {
  user: Pubkey,
  amount: String,
  claimed: String,
  user_total_staked: String,
  user_total_claimed: String,
  pool_total_staked: String,
  pool_total_claimed: String,
}


pub fn exec(ctx: Context<Withdraw>, amount: u64, _test_ts: i64) -> Result<()> {
  #[cfg(not(feature = "localnet"))]
  let now = Clock::get().unwrap().unix_timestamp;
  #[cfg(feature = "localnet")]
  let now = _test_ts;

  let user_info = &ctx.accounts.user_info;
  require!(user_info.staked_amount >= amount, ErrorCode::InsufficientWithdrawAmount);
  update_pool(&mut ctx.accounts.pool_info, now)?;

  let claimed = release_pending(AccountContainer {
    user: ctx.accounts.user.to_account_info(),
    user_info: &mut ctx.accounts.user_info,
    pool_info: &mut ctx.accounts.pool_info,
  })?;


  
  if amount > 0 {
    let user_info = &mut ctx.accounts.user_info;
    let pool_info = &mut ctx.accounts.pool_info;

    pool_info.total_staked = pool_info.total_staked.safe_sub(amount)?;
    user_info.staked_amount = user_info.staked_amount.safe_sub(amount)?;

    withdraw_stake(&ctx, amount)?;
  }

  let pool_info = &mut ctx.accounts.pool_info;
  let acc_reward_per_share = pool_info.acc_reward_per_share;
  let user_info = &mut ctx.accounts.user_info;

  user_info.reward_debt = user_info.staked_amount
    .safe_mul(acc_reward_per_share)?
    .safe_div(NORMALIZATION_FACTOR)?;

  emit!(WithdrawEvent {
    user: ctx.accounts.user.key(),
    amount: amount.to_string(),
    claimed: claimed.to_string(),
    user_total_staked: user_info.staked_amount.to_string(),
    user_total_claimed: user_info.total_claimed.to_string(),
    pool_total_staked: pool_info.total_staked.to_string(),
    pool_total_claimed: pool_info.total_claimed.to_string(),
  });

  Ok(())
}

fn withdraw_stake(ctx: &Context<Withdraw>, amount: u64) -> Result<()> {
  let cpi_accounts = TransferChecked {
    from: ctx.accounts.staking_token_vault_ata.to_account_info(),
    mint: ctx.accounts.staking_token.to_account_info(),
    to: ctx.accounts.user_staking_ata.to_account_info(),
    authority: ctx.accounts.pool_info.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

  token_2022::transfer_checked(cpi_ctx, amount, TOKEN_DECIMALS)?;

  Ok(())
}
