use anchor_lang::{
  prelude::*,
  solana_program::{
    instruction::Instruction, program::invoke,
  },
};
use anchor_safe_math::SafeMath;
use crate::{instructions::swap::Swap, processors::common::{collect_fees, create_treasury_atas}, raydium};

#[event]
pub struct SwapBaseOutputEvent {
  pub amount_sold: u64,
  pub amount_received: u64,
  pub user: Pubkey,
  pub input_token: Pubkey,
  pub output_token: Pubkey,
}

fn swap(ctx: &Context<Swap>, max_amount_in: u64, amount_out_less_fee: u64) -> Result<()> {
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
  let mut data: Vec<u8> = vec![55, 217, 98, 86, 163, 74, 180, 173];
  let mut ix_data: Vec<u8> = Vec::new();
  raydium::SwapBaseOutputIx {max_amount_in, amount_out_less_fee}.serialize(&mut ix_data)?;

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
  max_amount_in: u64,
  amount_out_less_fee: u64,
) -> Result<()> {
  create_treasury_atas(&ctx)?;

  let input_token_balance_before = ctx.accounts.input_token_account.amount;
  swap(&ctx, max_amount_in, amount_out_less_fee)?;
  ctx.accounts.input_token_account.reload()?;
  let input_token_balance_after = ctx.accounts.input_token_account.amount;

  collect_fees(&ctx, amount_out_less_fee)?;
  
  let amount_sold = input_token_balance_before.safe_sub(input_token_balance_after)?;

  emit_cpi!(SwapBaseOutputEvent {
    amount_sold,
    amount_received: amount_out_less_fee,
    user: ctx.accounts.payer.key(),
    input_token: ctx.accounts.input_token_mint.key(),
    output_token: ctx.accounts.output_token_mint.key(),
  });

  Ok(())
}
