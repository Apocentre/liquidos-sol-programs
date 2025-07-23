use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use anchor_spl::token_2022::{self, TransferChecked};
use onlybags::{account_data::bonding_curve::{self, BondingCurve}, processors::common::deser};
use crate::{
  instructions::deposit::Deposit, program_error::ErrorCode,
  staking::{release_pending, update_pool, AccountContainer, NORMALIZATION_FACTOR, TOKEN_DECIMALS},
};

#[event]
pub struct DepositEvent {
  user: Pubkey,
  reward_token: Pubkey,
  staking_token: Pubkey,
  amount: String,
  claimed: String,
  user_total_staked: String,
  user_total_claimed: String,
  pool_total_staked: String,
  pool_total_claimed: String,
}

fn lock_stake(ctx: &Context<Deposit>, amount: u64) -> Result<()> {
  let cpi_accounts = TransferChecked {
    from: ctx.accounts.user_staking_ata.to_account_info(),
    mint: ctx.accounts.staking_token.to_account_info(),
    to: ctx.accounts.staking_token_vault_ata.to_account_info(),
    authority: ctx.accounts.user.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

  token_2022::transfer_checked(cpi_ctx, amount, TOKEN_DECIMALS)?;

  Ok(())
}

pub fn exec(ctx: Context<Deposit>, amount: u64) -> Result<()> {
  let mut pool_info = ctx.accounts.pool_info.load_mut()?;
  let state = ctx.accounts.state.load()?;

  let now = Clock::get().unwrap().unix_timestamp;

  if pool_info.first_stake_ts == 0 {
    pool_info.first_stake_ts = now;
    pool_info.end_ts = now + pool_info.staking_duration;
  }

  require!(now >= pool_info.start_ts, ErrorCode::PoolNotStarted);
  require!(now <= pool_info.end_ts, ErrorCode::PoolEnded);

  update_pool(&mut *pool_info)?;
  let bonding_curve: BondingCurve = deser(ctx.accounts.bonding_curve.clone())?;

  let claimed = release_pending(
    bonding_curve.is_complete(),
    &mut AccountContainer {
      state: &*state,
      state_key: ctx.accounts.state.key(),
      user_info: &mut ctx.accounts.user_info,
      pool_info: &mut *pool_info,
      reward_token: &ctx.accounts.reward_token,
      reward_token_vault_ata: &ctx.accounts.reward_token_vault_ata,
      pool_authority: &ctx.accounts.pool_authority,
      user_reward_ata: &ctx.accounts.user_reward_ata,
      treasury_ata: &ctx.accounts.treasury_ata,
      token_2022: &ctx.accounts.token_2022,
    },
  )?;
  
  if amount > 0 {
    let user_info = &mut ctx.accounts.user_info;
    
    pool_info.total_staked = pool_info.total_staked.safe_add(amount)?;
    user_info.staked_amount = user_info.staked_amount.safe_add(amount)?;

    lock_stake(&ctx, amount)?;
  }

  let acc_reward_per_share = pool_info.acc_reward_per_share;
  let reward_token = pool_info.reward_token;
  let user_info = &mut ctx.accounts.user_info;

  user_info.reward_debt = user_info.staked_amount
  .safe_mul(acc_reward_per_share)?
  .safe_div(NORMALIZATION_FACTOR)?;

  emit_cpi!(DepositEvent {
    user: ctx.accounts.user.key(),
    reward_token,
    staking_token: pool_info.staking_token,
    amount: amount.to_string(),
    claimed: claimed.to_string(),
    user_total_staked: user_info.staked_amount.to_string(),
    user_total_claimed: user_info.total_claimed.to_string(),
    pool_total_staked: pool_info.total_staked.to_string(),
    pool_total_claimed: pool_info.total_claimed.to_string(),
  });

  Ok(())
}
