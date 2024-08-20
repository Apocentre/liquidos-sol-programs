use anchor_lang::prelude::*;
use crate::account_data::{pool_info::PoolInfo, user_info::UserInfo};

#[derive(Accounts)]
#[instruction(user: Pubkey, state: Pubkey, reward_token: Pubkey, staking_token: Pubkey)]
pub struct GetPendingReward<'info> {
  #[account(
    seeds = [b"staking_pool", state.as_ref(), reward_token.as_ref()],
    bump,
  )]
  pub pool_info: Account<'info, PoolInfo>,

  #[account(
    seeds = [b"user_info", user.as_ref(), state.as_ref(), staking_token.as_ref()],
    bump
  )]
  pub user_info: Account<'info, UserInfo>,
}
