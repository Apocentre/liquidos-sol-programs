use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};
use crate::account_data::{pool_info::PoolInfo, state::State};

#[derive(Accounts)]
pub struct CreatePool<'info> {
  /// The state account of each instance of this program
  #[account(mut)]
  pub state: AccountLoader<'info, State>,
  
  #[account(
    init,
    payer = payer,
    space = PoolInfo::MAX_SIZE,
    seeds = [b"staking_pool", state.key().as_ref(), reward_token.key().as_ref()],
    bump,
  )]
  pub pool_info: AccountLoader<'info, PoolInfo>,

  #[account()]
  pub reward_token: InterfaceAccount<'info, Mint>,

  /// CHECK: This is the authority of all the ATA that will store the staked tokens
  #[account(
    seeds = [b"pool_authority", state.key().as_ref()],
    bump = state.load()?.pool_authority_bump,
  )]
  pub pool_authority: AccountInfo<'info>,

  /// ATA that will store the reward tokens
  #[account(
    init_if_needed,
    payer = payer,
    associated_token::mint = reward_token,
    associated_token::authority = pool_authority,
    associated_token::token_program = token_2022,
  )]
  pub reward_token_vault_ata: InterfaceAccount<'info, TokenAccount>,

  /// The state of the bonding curve that will be used during buys and sells
  #[account(
    mut,
    seeds = [b"bonding_curve", state.load()?.onlybags_state.as_ref(), reward_token.key().as_ref()],
    bump,
  )]
  pub bonding_curve: Signer<'info>,
  /// This is the user the call the instruction which calls this instruction via CPI. We need this to pay
  /// for the rent for the above created accounts
  #[account(mut)]
  pub payer: Signer<'info>,
  pub token_2022: Interface<'info, TokenInterface>, 
  associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
}
