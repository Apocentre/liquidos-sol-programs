use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken, token_interface::{TokenAccount, TokenInterface, Mint},
};
use crate::account_data::state::State;

#[derive(Accounts)]
pub struct Initialize<'info> {
  /// The state account of each instance of this program
  #[account(
    init,
    payer = owner,
    space = State::MAX_SIZE,
  )]
  pub state: Account<'info, State>,

  /// CHECK: This is the authority that will control the ATA of each pool and execute CPIs
  #[account(
    init,
    space = 0,
    payer = owner,
    seeds = [b"pool_authority", state.key().as_ref()],
    bump,
  )]
  pub pool_authority: AccountInfo<'info>,

  #[account(mut)]
  pub staking_token: InterfaceAccount<'info, Mint>,

  /// ATA that will store the staking tokens for all pools
  #[account(
    init,
    payer = owner,
    associated_token::mint = staking_token,
    associated_token::authority = pool_authority,
    associated_token::token_program = token_2022,
  )]
  pub staking_token_vault_ata: InterfaceAccount<'info, TokenAccount>,
  
  #[account(mut)]
  pub owner: Signer<'info>,
  
  pub token_2022: Interface<'info, TokenInterface>, 
  associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
}
