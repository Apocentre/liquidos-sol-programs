use anchor_lang::{
  prelude::*,
  solana_program::{
    instruction::Instruction, program::invoke,
  },
};
use anchor_safe_math::SafeMath;
use crate::{instructions::swap::Swap, processors::common::{collect_fees, create_treasury_atas}, raydium};

#[event]
pub struct SwapBaseInputEvent {
  pub amount_sold: u64,
  pub amount_received: u64,
  pub user: Pubkey,
  pub input_token: Pubkey,
  pub output_token: Pubkey,
}

fn swap(ctx: &Context<Swap>, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
  let accounts = vec![
    AccountMeta::new(ctx.accounts.payer.key(), true),
    AccountMeta::new_readonly(ctx.accounts.raydium_authority.key(), false),
    AccountMeta::new_readonly(ctx.accounts.amm_config.key(), false),
    AccountMeta::new(ctx.accounts.pool_state.key(), false),
    AccountMeta::new(ctx.accounts.input_token_account.key(), false),
    AccountMeta::new(ctx.accounts.output_token_account.key(), false),
    AccountMeta::new(ctx.accounts.input_vault.key(), false),
    AccountMeta::new(ctx.accounts.output_vault.key(), false),
    AccountMeta::new_readonly(ctx.accounts.input_token_program.key(), false),
    AccountMeta::new_readonly(ctx.accounts.output_token_program.key(), false),
    AccountMeta::new_readonly(ctx.accounts.input_token_mint.key(), false),
    AccountMeta::new_readonly(ctx.accounts.output_token_mint.key(), false),
    AccountMeta::new(ctx.accounts.observation_state.key(), false),
  ];

  // add the ix_discriminator so Raydium's Anchor program can identity the instruction
  let mut data: Vec<u8> = vec![143, 190, 90, 218, 196, 30, 51, 222];
  let mut ix_data: Vec<u8> = Vec::new();
  raydium::SwapBaseInputIx {amount_in, minimum_amount_out}.serialize(&mut ix_data)?;

  data.extend(&ix_data);
  let ix = Instruction {
    program_id: ctx.accounts.cp_swap_program.key(),
    accounts,
    data,
  };

  invoke(
    &ix,
    &[
      ctx.accounts.payer.to_account_info(),
      ctx.accounts.raydium_authority.to_account_info(),
      ctx.accounts.amm_config.to_account_info(),
      ctx.accounts.pool_state.to_account_info(),
      ctx.accounts.input_token_account.to_account_info(),
      ctx.accounts.output_token_account.to_account_info(),
      ctx.accounts.input_vault.to_account_info(),
      ctx.accounts.output_vault.to_account_info(),
      ctx.accounts.input_token_program.to_account_info(),
      ctx.accounts.output_token_program.to_account_info(),
      ctx.accounts.input_token_mint.to_account_info(),
      ctx.accounts.output_token_mint.to_account_info(),
      ctx.accounts.observation_state.to_account_info(),
    ]
  )?;

  Ok(())
}


pub fn exec<'info>(
  ctx: Context<'_, '_, '_, 'info, Swap<'info>>,
  amount_in: u64,
  minimum_amount_out: u64,
) -> Result<()> {
  create_treasury_atas(&ctx)?;

  let output_token_balance_before = ctx.accounts.output_token_account.amount;
  swap(&ctx, amount_in, minimum_amount_out)?;
  ctx.accounts.output_token_account.reload()?;
  let output_token_balance_after = ctx.accounts.output_token_account.amount;

  let amount_received = output_token_balance_after.safe_sub(output_token_balance_before)?;
  collect_fees(&ctx, amount_received)?;
  
  emit_cpi!(SwapBaseInputEvent {
    amount_sold: amount_in,
    amount_received,
    user: ctx.accounts.payer.key(),
    input_token: ctx.accounts.input_token_mint.key(),
    output_token: ctx.accounts.output_token_mint.key(),
  });

  Ok(())
}
