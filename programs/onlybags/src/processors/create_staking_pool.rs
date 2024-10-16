
use anchor_lang::{
  prelude::{borsh::BorshSerialize, *},
  solana_program::{
    instruction::Instruction, program::invoke_signed,
  },
};
use anchor_spl::token_2022::{MintTo, mint_to};
use crate::instructions::create_staking_pool::CreateStakingPool;

#[derive(BorshSerialize)]
pub struct CreatePoolIx {
  pub total_rewards: u64,
}

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

pub fn exec(ctx: Context<CreateStakingPool>) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;
  
  // This Ix might be called even if the pool is completed. Read the docs of `instrospect_next_ix` for more details.
  // We want to act upon only if the curce is completed
  if curve.closed == 1 {
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
  }

  Ok(())
}
