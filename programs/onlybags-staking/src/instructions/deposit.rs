use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  token_interface::{Mint, TokenAccount, TokenInterface}
};
use crate::{
  program_error::ErrorCode,
  account_data::{pool_info::PoolInfo, state::State},
};

#[derive(Accounts)]
pub struct Deposit<'info> {
  #[account()]
  pub state: Account<'info, State>,

  #[account(
    seeds = [b"staking_pool", state.key().as_ref(), reward_token.key().as_ref()],
    bump,
  )]
  pub pool_info: Account<'info, PoolInfo>,

  #[account()]
  pub reward_token: InterfaceAccount<'info, Mint>,

  /// CHECK: This is the authority of all the ATA that will store the staked tokens
  #[account(
    seeds = [b"pool_authority", state.key().as_ref()],
    bump = state.pool_authority_bump,
  )]
  pub pool_authority: AccountInfo<'info>,

  /// ATA that will store the reward tokens
  #[account(
    associated_token::mint = reward_token,
    associated_token::authority = pool_authority,
    associated_token::token_program = token_2022,
  )]
  pub reward_token_vault_ata: InterfaceAccount<'info, TokenAccount>,

  #[account(
    mut,
    constraint = state.staking_token.is_some() @ ErrorCode::StakingTokenNotSet,
    constraint = staking_token.key() == state.staking_token.unwrap() @ ErrorCode::InvalidStakingToken,
  )]
  pub staking_token: InterfaceAccount<'info, Mint>,

  /// ATA that will store the staking tokens for all pools
  #[account(
    associated_token::mint = staking_token,
    associated_token::authority = pool_authority,
    associated_token::token_program = token_2022,
  )]
  pub staking_token_vault_ata: InterfaceAccount<'info, TokenAccount>,
  
  pub token_2022: Interface<'info, TokenInterface>, 
  associated_token_program: Program<'info, AssociatedToken>,
}
