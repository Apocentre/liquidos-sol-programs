
use anchor_lang::{
  prelude::{borsh::BorshSerialize, *}, Discriminator,
  solana_program::{
    instruction::Instruction, program::invoke_signed,
    sysvar::instructions::{load_current_index_checked, load_instruction_at_checked},
  },
};
use anchor_spl::token_2022::{MintTo, mint_to};
use crate::{
  ID, instruction::{CreateTaxToken, CreateToken}, instructions::create_staking_pool::CreateStakingPool, program_error::ErrorCode
};

#[derive(BorshSerialize)]
pub struct CreatePoolIx {
  pub total_rewards: u64,
}

/// Creates a new staking pool. This will fail if we try to create a staking pool for the same meme coin
/// This is due to the PDA constraints of the pool_info which will create a deterministic address
/// for the same reward token and staking_pool_state.
fn create_pool(ctx: &Context<CreateStakingPool>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;

  let accounts = vec![
    AccountMeta::new(ctx.accounts.staking_state.key(), false),
    AccountMeta::new(ctx.accounts.pool_info.key(), false),
    AccountMeta::new_readonly(ctx.accounts.token.key(), false), // staking token
    AccountMeta::new_readonly(ctx.accounts.staking_token_vault_ata.key(), false),
    AccountMeta::new_readonly(ctx.accounts.token.key(), false), // reward token
    AccountMeta::new_readonly(ctx.accounts.pool_authority.key(), false),
    AccountMeta::new(ctx.accounts.reward_token_vault_ata.key(), false),
    AccountMeta::new(ctx.accounts.bonding_curve.key(), true),
    AccountMeta::new(ctx.accounts.payer.key(), true),
    AccountMeta::new_readonly(ctx.accounts.token_2022.key(), false),
    AccountMeta::new_readonly(ctx.accounts.associated_token_program.key(), false),
    AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
  ];

  // add the ix_discriminator so Staking Anchor program can identity the instruction
  let mut data: Vec<u8> = vec![233, 146, 209, 142, 207, 104, 64, 188];
  let mut ix_data: Vec<u8> = Vec::new();

  CreatePoolIx {
    total_rewards: curve.calc_staking_allocation()?,
  }.serialize(&mut ix_data)?;

  data.extend(&ix_data);
  let ix = Instruction {
    program_id: ctx.accounts.staking_program.key(),
    accounts,
    data,
  };

  invoke_signed(
    &ix,
    &[
      ctx.accounts.staking_state.to_account_info(),
      ctx.accounts.pool_info.to_account_info(),
      ctx.accounts.token.to_account_info(),
      ctx.accounts.staking_token_vault_ata.to_account_info(),
      ctx.accounts.pool_authority.to_account_info(),
      ctx.accounts.reward_token_vault_ata.to_account_info(),
      ctx.accounts.bonding_curve.to_account_info(),
      ctx.accounts.payer.to_account_info(),
      ctx.accounts.token_2022.to_account_info(),
      ctx.accounts.associated_token_program.to_account_info(),
      ctx.accounts.system_program.to_account_info(),
    ],
    signer_seeds,
  )?;

  Ok(())
}

fn tranfer_rewards_to_pool(ctx: &Context<CreateStakingPool>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;
  let total_rewards = curve.calc_staking_allocation()?;

  let cpi_accounts = MintTo {
    mint: ctx.accounts.token.to_account_info(),
    to: ctx.accounts.reward_token_vault_ata.to_account_info(),
    authority: ctx.accounts.bonding_curve.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  mint_to(cpi_ctx, total_rewards)?;

  Ok(())
}

/// NOTE! One can dos the creation of the token by first calling this ix and then the create_token
/// which will end up calling this ix again and thus the tx will fail. We can circumvent this by adding
/// ix introspection i.e. making sure the previous tx is a create_token/create_tax_token.
fn instrospect_prev_ix(ctx: &Context<CreateStakingPool>) -> Result<()> {
  if ctx.accounts.bonding_curve.staking_allocation > 0 {
    let current_index = load_current_index_checked(&ctx.accounts.ix_sysvar.to_account_info())?;

    // check previous ix
    let current_ix = load_instruction_at_checked((current_index - 1) as usize, &ctx.accounts.ix_sysvar.to_account_info())?;
    require!(current_ix.program_id.eq(&ID), ErrorCode::WrongProgramId);
    let discriminator: [u8; 8] = current_ix.data[..8].try_into().map_err(|_| ErrorCode::WrongIxData)?;

    require!(
      discriminator.eq(&CreateToken::DISCRIMINATOR) || discriminator.eq(&CreateTaxToken::DISCRIMINATOR),
      ErrorCode::ExpectedCreateStakingPoolIx,
    );
  }

  Ok(())
}

pub fn exec(ctx: Context<CreateStakingPool>) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;
  require!(curve.staking_allocation > 0, ErrorCode::CannotCreateStakingPool);
  instrospect_prev_ix(&ctx)?;

  let state_key = &ctx.accounts.state.key();
  let token_key = &ctx.accounts.token.key();
  let seeds: &[&[u8]] = &[
    b"bonding_curve",
    state_key.as_ref(),
    token_key.as_ref(),
    &[curve.bump],
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  create_pool(&ctx, signer_seeds)?;
  tranfer_rewards_to_pool(&ctx, signer_seeds)?;

  Ok(())
}
