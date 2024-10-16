use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  token_interface::{Mint, TokenAccount, TokenInterface}
};
use crate::{
  account_data::{pool_info::PoolInfo, state::State, user_info::UserInfo}, program_error::ErrorCode
};

#[derive(Accounts)]
pub struct Deposit<'info> {
  #[account()]
  pub state: AccountLoader<'info, State>,

  #[account(
    mut,
    seeds = [b"staking_pool", state.key().as_ref(), reward_token.key().as_ref()],
    bump,
  )]
  pub pool_info: AccountLoader<'info, PoolInfo>,

  #[account()]
  pub reward_token: Box<InterfaceAccount<'info, Mint>>,

  /// CHECK: This is the authority of all the ATA that will store the staked tokens
  #[account(
    mut,
    seeds = [b"pool_authority", state.key().as_ref()],
    bump = state.load()?.pool_authority_bump,
  )]
  pub pool_authority: AccountInfo<'info>,

  /// CHECK: This is the authority of all the ATA that will store the staked tokens
  #[account(
    constraint = treasury.key() == state.load()?.treasury @ ErrorCode::InvalidTreasury,
  )]
  pub treasury: AccountInfo<'info>,

  /// ATA that will store the reward tokens
  #[account(
    init_if_needed,
    payer = user,
    associated_token::mint = reward_token,
    associated_token::authority = treasury,
    associated_token::token_program = token_2022,
  )]
  pub treasury_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  /// ATA that will store the reward tokens
  #[account(
    mut,
    associated_token::mint = reward_token,
    associated_token::authority = pool_authority,
    associated_token::token_program = token_2022,
  )]
  pub reward_token_vault_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  #[account(
    constraint = staking_token.key() == pool_info.load()?.staking_token @ ErrorCode::InvalidStakingToken,
  )]
  pub staking_token: Box<InterfaceAccount<'info, Mint>>,

  /// ATA that will store the staking tokens for all pools
  #[account(
    mut,
    associated_token::mint = staking_token,
    associated_token::authority = pool_authority,
    associated_token::token_program = token_2022,
  )]
  pub staking_token_vault_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  #[account(
    mut,
    seeds = [b"user_info", user.key().as_ref(), state.key().as_ref(), reward_token.key().as_ref()],
    bump
  )]
  pub user_info: Box<Account<'info, UserInfo>>,

  #[account(
    mut,
    associated_token::mint = staking_token,
    associated_token::authority = user,
    associated_token::token_program = token_2022,
  )]
  pub user_staking_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  #[account(
    mut,
    associated_token::mint = reward_token,
    associated_token::authority = user,
    associated_token::token_program = token_2022,
  )]
  pub user_reward_ata: Box<InterfaceAccount<'info, TokenAccount>>,
  
  #[account(mut)]
  pub user: Signer<'info>,
  pub token_2022: Interface<'info, TokenInterface>, 
  associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
}
