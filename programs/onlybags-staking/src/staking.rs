use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use anchor_spl::{
  token_2022::{self, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface},
};
use crate::account_data::{pool_info::PoolInfo, state::State, user_info::UserInfo};

pub const NORMALIZATION_FACTOR: u64 = 1_000_000;
pub const TOKEN_DECIMALS: u8 = 6;

pub struct AccountContainer<'a, 'info> {
  pub state: &'a State,
  pub state_key: Pubkey,
  pub user_info: &'a mut Box<Account<'info, UserInfo>>,
  pub pool_info: &'a mut PoolInfo,
  pub reward_token: &'a Box<InterfaceAccount<'info, Mint>>,
  pub reward_token_vault_ata: &'a Box<InterfaceAccount<'info, TokenAccount>>,
  pub pool_authority: &'a AccountInfo<'info>,
  pub user_reward_ata: &'a Box<InterfaceAccount<'info, TokenAccount>>,
  pub treasury_ata: &'a Box<InterfaceAccount<'info, TokenAccount>>,
  pub token_2022: &'a Interface<'info, TokenInterface>,
}

pub fn get_pending_rewards<'info>(
  pool_info: &PoolInfo,
  user_info: &Account<'info, UserInfo>,
) -> Result<u64> {  
  let now = Clock::get().unwrap().unix_timestamp;
  let acc_reward_per_share = calc_acc_reward_per_share(pool_info, now)?;

  let pending = user_info.staked_amount
  .safe_mul(acc_reward_per_share)?
  .safe_div(NORMALIZATION_FACTOR)?
  .safe_sub(user_info.reward_debt)?
  .safe_add(user_info.acc_claim)?;

  Ok(pending)
}

pub fn update_pool(pool_info: &mut PoolInfo) -> Result<()> {
  let now = Clock::get().unwrap().unix_timestamp;

  if now <= pool_info.last_reward_ts {
    return Ok(())
  }

  if pool_info.total_staked > 0 {
    pool_info.acc_reward_per_share = calc_acc_reward_per_share(pool_info, now)?;
  }

  pool_info.last_reward_ts = now.min(pool_info.end_ts);

  Ok(())
}

pub fn release_pending(accounts: &mut AccountContainer) -> Result<u64>{
  let user_info = &mut accounts.user_info;
  let pool_info =  &mut accounts.pool_info;
  let now = Clock::get().unwrap().unix_timestamp;
  let mut amount = 0;

  if user_info.staked_amount > 0 {
    let pending = user_info.staked_amount
    .safe_mul(pool_info.acc_reward_per_share)?
    .safe_div(NORMALIZATION_FACTOR)?
    .safe_sub(user_info.reward_debt)?;
    let total_pending = pending.safe_add(user_info.acc_claim)?;
    
    if now > pool_info.timelock_ts && total_pending > 0 {
      let (user_amount, treasury_amount) = split_rewards(pool_info.protocol_fee, total_pending)?;

      amount = user_amount;
      user_info.acc_claim = 0;
      user_info.total_claimed = user_info.total_claimed.safe_add(user_amount)?;
      pool_info.total_claimed = pool_info.total_claimed.safe_add(user_amount)?;

      transfer_rewards(
        accounts, 
        user_amount,
        treasury_amount,
      )?;
    } else if pending > 0 {
      user_info.acc_claim = total_pending;
    }
  } else if now > pool_info.timelock_ts && user_info.acc_claim > 0 {
    let acc_claim = user_info.acc_claim;
    user_info.acc_claim = 0;

    let (user_amount, treasury_amount) = split_rewards(pool_info.protocol_fee, acc_claim)?;
    amount = user_amount;
    user_info.total_claimed = user_info.total_claimed.safe_add(user_amount)?;
    pool_info.total_claimed = pool_info.total_claimed.safe_add(user_amount)?;

    transfer_rewards(
      accounts, 
      user_amount,
      treasury_amount,
    )?;
  }

  Ok(amount)
} 

fn calc_pending_reward(pool_info: &PoolInfo, now: i64) -> Result<u64> {
  let time_elapsed = (now.min(pool_info.end_ts) - pool_info.last_reward_ts) as u64;
  let reward = time_elapsed.safe_mul(pool_info.reward_per_sec)?;

  Ok(reward as u64)
}

fn calc_acc_reward_per_share(pool_info: &PoolInfo, now: i64) -> Result<u64> {
  let pending_reward = calc_pending_reward(pool_info, now)?;
  let acc_reward_per_share = pending_reward
  .safe_mul(NORMALIZATION_FACTOR)?
  .safe_div(pool_info.total_staked as u64)?;

  let new_acc_reward_per_share = pool_info.acc_reward_per_share.safe_add(acc_reward_per_share)?;

  Ok(new_acc_reward_per_share)
}

fn split_rewards(protocol_fee: u16, amount: u64) -> Result<(u64, u64)> {
  let treasury_amount = amount
  .safe_mul(protocol_fee as u64)?
  .safe_div(10_000)?;

  let user_amount = amount.safe_sub(treasury_amount)?;

  Ok((user_amount, treasury_amount))
}

fn transfer_rewards<'info>(
  accounts: &mut AccountContainer,
  user_amount: u64,
  treasury_amount: u64
) -> Result<()> {
  let seeds: &[&[u8]] = &[
    b"pool_authority",
    accounts.state_key.as_ref(),
    &[accounts.state.pool_authority_bump],
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  // send user rewards
  let cpi_accounts = TransferChecked {
    from: accounts.reward_token_vault_ata.to_account_info(),
    mint: accounts.reward_token.to_account_info(),
    to: accounts.user_reward_ata.to_account_info(),
    authority: accounts.pool_authority.to_account_info(),
  };
  let cpi_program = accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  token_2022::transfer_checked(cpi_ctx, user_amount, TOKEN_DECIMALS)?;

  // collect protocol fees
  let cpi_accounts = TransferChecked {
    from: accounts.reward_token_vault_ata.to_account_info(),
    mint: accounts.reward_token.to_account_info(),
    to: accounts.treasury_ata.to_account_info(),
    authority: accounts.pool_authority.to_account_info(),
  };
  let cpi_program = accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  token_2022::transfer_checked(cpi_ctx, treasury_amount, TOKEN_DECIMALS)?;

  Ok(())
}
