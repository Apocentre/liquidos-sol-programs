use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface},
};
use crate::{
  account_data::{pool_info::PoolInfo, state::State},
  constants::{allowed_deployer, liquidos_token}, program_error::ErrorCode,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
  /// The state account of each instance of this program
  #[account(
    init,
    payer = deployer,
    space = State::MAX_SIZE,
  )]
  pub state: Account<'info, State>,

  #[account(
    init,
    payer = deployer,
    space = PoolInfo::MAX_SIZE,
    seeds = [b"staking_pool", state.key().as_ref()],
    bump,
  )]
  pub pool_info: Account<'info, PoolInfo>,

  #[account(
    address = liquidos_token() @ ErrorCode::InvalidStakingToken,
  )]
  pub staking_token: Box<InterfaceAccount<'info, Mint>>,

  /// ATA that will store the staking tokens for this pool
  #[account(
    init,
    payer = deployer,
    associated_token::mint = staking_token,
    associated_token::authority = pool_info,
    associated_token::token_program = token_2022,
  )]
  pub staking_token_vault_ata: Box<InterfaceAccount<'info, TokenAccount>>,
  
  #[account(
    mut,
    address = allowed_deployer() @ ErrorCode::WrongDeployer,
  )]
  pub deployer: Signer<'info>,
  
  pub token_2022: Interface<'info, TokenInterface>,
  associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
}
