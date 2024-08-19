use anchor_lang::prelude::*;
use crate::{
  account_data::{pool_info::PoolInfo, user_info::UserInfo},
  instructions::deposit::Deposit, staking::update_pool
};

pub fn exec(ctx: Context<Deposit>, amount: u64) -> Result<()> {
  let state = &ctx.accounts.state;
  let pool_info =  &mut ctx.accounts.pool_info;

  update_pool(pool_info)?;

  Ok(())
}
