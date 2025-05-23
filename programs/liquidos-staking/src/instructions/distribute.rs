use anchor_lang::prelude::*;
use crate::{
  account_data::{pool_info::PoolInfo, state::State},
};

#[derive(Accounts)]
pub struct Distribute<'info> {
  #[account()]
  pub state: Account<'info, State>,

  #[account(
    mut,
    seeds = [b"staking_pool", state.key().as_ref()],
    bump = pool_info.bump,
  )]
  pub pool_info: Account<'info, PoolInfo>,

  
  // TODO: add check to make sure only allowed distributors can call this ix
  #[account()]
  pub distributor: Signer<'info>,
}
