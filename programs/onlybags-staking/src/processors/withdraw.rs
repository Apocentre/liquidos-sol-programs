use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use anchor_spl::token_2022::{self, TransferChecked};
use crate::{
  instructions::withdraw::Withdraw, program_error::ErrorCode,
  staking::{release_pending, update_pool, AccountContainer, NORMALIZATION_FACTOR, TOKEN_DECIMALS},
};

fn unlock_stake(ctx: &Context<Withdraw>, amount: u64) -> Result<()> {
  let state_key = ctx.accounts.state.key();
  let seeds: &[&[u8]] = &[
    b"pool_authority",
    state_key.as_ref(),
    &[ctx.accounts.state.pool_authority_bump],
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  let cpi_accounts = TransferChecked {
    from: ctx.accounts.staking_token_vault_ata.to_account_info(),
    mint: ctx.accounts.staking_token.to_account_info(),
    to: ctx.accounts.user_staking_ata.to_account_info(),
    authority: ctx.accounts.user.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  token_2022::transfer_checked(cpi_ctx, amount, TOKEN_DECIMALS)?;

  Ok(())
}

pub fn exec(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
  let pool_info = &mut ctx.accounts.pool_info;
  let user_info = &ctx.accounts.user_info;
  
  require!(user_info.staked_amount >= amount, ErrorCode::InsufficientWithdrawAmount,);

  update_pool(pool_info)?;
  release_pending(&mut AccountContainer {
    state: &mut ctx.accounts.state,
    user_info: &mut ctx.accounts.user_info,
    pool_info: &mut ctx.accounts.pool_info,
    reward_token: &ctx.accounts.reward_token,
    reward_token_vault_ata: &ctx.accounts.reward_token_vault_ata,
    pool_authority: &ctx.accounts.pool_authority,
    user_reward_ata: &ctx.accounts.user_reward_ata,
    treasury_ata: &ctx.accounts.treasury_ata,
    token_2022: &ctx.accounts.token_2022,
  })?;

  let pool_info =  &mut ctx.accounts.pool_info;
  let acc_reward_per_share = pool_info.acc_reward_per_share;

  if amount > 0 {
    let user_info = &mut ctx.accounts.user_info;

    pool_info.total_staked = pool_info.total_staked.safe_sub(amount)?;
    user_info.staked_amount = user_info.staked_amount.safe_sub(amount)?;

    unlock_stake(&ctx, amount)?;
  }

  let user_info = &mut ctx.accounts.user_info;
  user_info.reward_debt = user_info.staked_amount
  .safe_mul(acc_reward_per_share)?
  .safe_div(NORMALIZATION_FACTOR)?;

  Ok(())
}
