use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface},
};
use crate::{
  account_data::{pool_info::PoolInfo, state::State, user_info::UserInfo}, constants::liquidos_token, program_error::ErrorCode
};

#[derive(Accounts)]
pub struct Deposit<'info> {
  #[account()]
  pub state: Account<'info, State>,

  #[account(
    mut,
    seeds = [b"staking_pool", state.key().as_ref()],
    bump = pool_info.bump,
  )]
  pub pool_info: Account<'info, PoolInfo>,

  #[account(
    init_if_needed,
    payer = user,
    space = UserInfo::MAX_SIZE,
    seeds = [b"user_info", user.key().as_ref(), pool_info.key().as_ref()],
    bump
  )]
  pub user_info: Account<'info, UserInfo>,

    #[account(
    address = liquidos_token() @ ErrorCode::InvalidStakingToken,
  )]
  pub staking_token: Box<InterfaceAccount<'info, Mint>>,

  #[account(
    mut,
    associated_token::mint = staking_token,
    associated_token::authority = user,
    associated_token::token_program = token_2022,
  )]
  pub user_staking_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  #[account(
    associated_token::mint = staking_token,
    associated_token::authority = pool_info,
    associated_token::token_program = token_2022,
  )]
  pub staking_token_vault_ata: Box<InterfaceAccount<'info, TokenAccount>>,
  
  #[account(mut)]
  pub user: Signer<'info>,
  pub token_2022: Interface<'info, TokenInterface>,
  associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
}
