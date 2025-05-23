use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use crate::{
  account_data::{pool_info::PoolInfo, user_info::UserInfo}, program_error::ErrorCode,
};

pub const NORMALIZATION_FACTOR: u64 = 1_000_000;
pub const TOKEN_DECIMALS: u8 = 6;

pub fn harvest(pool_info: &mut PoolInfo, now: i64) -> Result<()> {
  let last_harvest_ts = pool_info.last_harvest_ts;
  let round_end_ts = pool_info.round_end_ts;

  require!(now != last_harvest_ts, ErrorCode::SameBlockHarvest);
  require!(last_harvest_ts != round_end_ts, ErrorCode::RoundEnd);

  if now < round_end_ts {
    let acc_reward = ((now - last_harvest_ts) as u64).safe_mul(pool_info.reward_per_sec)?;
    pool_info.pending_reward = pool_info.pending_reward.safe_add(acc_reward)?;
    pool_info.last_harvest_ts = now;
  } else {
    let acc_reward = ((round_end_ts - last_harvest_ts) as u64).safe_mul(pool_info.reward_per_sec)?;
    pool_info.pending_reward = pool_info.pending_reward.safe_add(acc_reward)?;
    pool_info.last_harvest_ts = round_end_ts;
  }

  Ok(())
}

pub fn update_pool(pool_info: &mut PoolInfo, now: i64) -> Result<()> {
  harvest(pool_info, now)?;

  if pool_info.total_staked > 0 && pool_info.pending_reward > 0 {
    pool_info.acc_reward_per_share = calc_acc_reward_per_share(pool_info)?;
    pool_info.pending_reward = 0;
  }

  Ok(())
}

fn calc_acc_reward_per_share(pool_info: &PoolInfo) -> Result<u64> {
  let acc_reward_per_share = pool_info.pending_reward
    .safe_mul(NORMALIZATION_FACTOR)?
    .safe_div(pool_info.total_staked as u64)?;
  let new_acc_reward_per_share = pool_info.acc_reward_per_share
    .safe_add(acc_reward_per_share)?;

  Ok(new_acc_reward_per_share)
}

pub struct AccountContainer<'a, 'info> {
  pub user: AccountInfo<'info>,
  pub user_info: &'a mut UserInfo,
  pub pool_info: &'a mut Account<'info, PoolInfo>,
}

pub fn release_pending(mut accounts: AccountContainer) -> Result<u64> {
  let user_info = accounts.user_info;
  let pool_info =  accounts.pool_info;

  if user_info.staked_amount == 0 {
    return Ok(0)
  }

  let claimed = user_info.staked_amount
    .safe_mul(pool_info.acc_reward_per_share)?
    .safe_div(NORMALIZATION_FACTOR)?
    .safe_sub(user_info.reward_debt)?;

  pool_info.total_claimed = pool_info.total_claimed.safe_add(claimed)?;
  user_info.total_claimed = user_info.total_claimed.safe_add(claimed)?;

  transfer_from_pda(
    &mut pool_info.to_account_info(),
    &mut accounts.user,
    claimed,
  )?;

  Ok(claimed)
}

/// We can do this because the from account is a pda which is owned by this program. Otherwise it will fail with
/// `failed to verify account ...: instruction spent from the balance of an account it does not own`
/// Note that the same error appears to be raised when we try to transfer like this
///
/// ```no_run
/// let cpi_context = CpiContext::new_with_signer(
///    ctx.accounts.system_program.to_account_info(),
///    NativeTransfer {from: escrow.clone(), to: treasury.clone()},
///    signer_seeds,
/// );
///  transfer(cpi_context, protocol_fee_amount)?;
/// ```
/// Neither `invoke_signed` works.
pub fn transfer_from_pda(
  from_pda: &mut AccountInfo,
  to: &mut AccountInfo,
  amount: u64,
) -> Result<()> {
  from_pda.sub_lamports(amount)?;
  to.add_lamports(amount)?;

  Ok(())
}
