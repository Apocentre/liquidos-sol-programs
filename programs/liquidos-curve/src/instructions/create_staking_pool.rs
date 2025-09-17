use anchor_lang::{prelude::*, solana_program::sysvar};
use anchor_spl::{
  token_interface::{TokenInterface, Mint},
  associated_token::AssociatedToken,
};
use crate::{
  account_data::{bonding_curve::BondingCurve, state::State},
  program_error::ErrorCode,
};

#[derive(Accounts)]
pub struct CreateStakingPool<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  /// The state account of each instance of this program
  #[account()]
  pub state: Box<Account<'info, State>>,

  /// The state of the bonding curve that will be used during buys and sells
  #[account(
    mut,
    seeds = [b"bonding_curve", state.key().as_ref(), token.key().as_ref()],
    bump = bonding_curve.bump,
  )]
  pub bonding_curve: Box<Account<'info, BondingCurve>>,
  
  #[account(
    mut,
    constraint = token.key() == bonding_curve.token @ ErrorCode::InvalidCurveToken,
  )]
  pub token: Box<InterfaceAccount<'info, Mint>>,

  // ---------------- Staking Program accounts ----------------
  /// CHECK: The staking program state. Checks will take place in Staking program
  #[account(
    mut,
    constraint = staking_state.key() == state.staking_program_state.unwrap() @ ErrorCode::WrongStakingProgramState,
  )]
  pub staking_state: AccountInfo<'info>,
  /// CHECK: The pool info. Checks will take place in Staking program
  #[account(mut)]
  pub pool_info: AccountInfo<'info>,
  /// CHECK: The pool authority. Checks will take place in Staking program
  #[account(mut)]
  pub pool_authority: AccountInfo<'info>,
  /// CHECK: The staking token of this pool. Checks will take place in Staking program
  #[account()]
  pub staking_token: AccountInfo<'info>,
  /// CHECK: ATA that will store the staking tokens for this pool. Checks will take place in Staking program
  #[account(mut)]
  pub staking_token_vault_ata: AccountInfo<'info>,
  /// CHECK: The pool PDA ata that will hold the tokens. Checks will take place in Staking program
  #[account(mut)]
  pub reward_token_vault_ata: AccountInfo<'info>,
  /// CHECK: The staking program id.
  #[account(
    constraint = staking_program.key() == state.staking_program.unwrap() @ ErrorCode::WrongStakingProgram,
  )]
  pub staking_program: AccountInfo<'info>,

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_2022: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,

  /// CHECK: custom constrain to check the correctness of the account address
  #[account(address = sysvar::instructions::id())]
  pub ix_sysvar: UncheckedAccount<'info>,
}

