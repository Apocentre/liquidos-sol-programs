use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  token_interface::{Mint, TokenAccount, TokenInterface}
};
use crate::{
  account_data::{state::State, user_info::UserInfo}, program_error::ErrorCode
};

#[derive(Accounts)]
pub struct InitUserInfo<'info> {
  #[account()]
  pub state: AccountLoader<'info, State>,

  #[account()]
  pub reward_token: Box<InterfaceAccount<'info, Mint>>,

  #[account(
    mut,
    constraint = state.load()?.staking_token != Pubkey::default() @ ErrorCode::StakingTokenNotSet,
    constraint = staking_token.key() == state.load()?.staking_token @ ErrorCode::InvalidStakingToken,
  )]
  pub staking_token: Box<InterfaceAccount<'info, Mint>>,

  #[account(
    init_if_needed,
    payer = user,
    space = UserInfo::MAX_SIZE,
    seeds = [b"user_info", user.key().as_ref(), state.key().as_ref(), reward_token.key().as_ref()],
    bump
  )]
  pub user_info: Box<Account<'info, UserInfo>>,

  #[account(
    init_if_needed,
    payer = user,
    associated_token::mint = staking_token,
    associated_token::authority = user,
    associated_token::token_program = token_2022,
  )]
  pub user_staking_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  #[account(
    init_if_needed,
    payer = user,
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
