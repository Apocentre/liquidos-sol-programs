use anchor_lang::prelude::*;
use crate::{
  account_data::{state::State, pool_info::PoolInfo},
  instructions::initialize::Initialize,
};

pub fn exec(ctx: Context<Initialize>) -> Result<()> {
  let owner = ctx.accounts.deployer.key();

  *ctx.accounts.state = State::new(
    owner,
    ctx.bumps.pool_authority,
  );
  *ctx.accounts.pool_info = PoolInfo::new(
    ctx.accounts.staking_token.key(),
  );

  Ok(())
}
