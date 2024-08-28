use anchor_lang::{
  prelude::*,
  solana_program::{
    instruction::Instruction, program::invoke,
  },
};
use anchor_safe_math::SafeMath;
use anchor_spl::{token::{self, Transfer}, token_2022::{self, TransferChecked}};
use crate::{instructions::swap::Swap, raydium::{self, is_wsol}};
use super::swap_base_input::TOKEN_DECIMALS;

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

fn collect_fees(ctx: &Context<Swap>, token_amount_received: u64) -> Result<()> {
  let state = &ctx.accounts.state;
  let input_token_mint = &ctx.accounts.input_token_mint;
  let fees = token_amount_received.safe_mul(state.protocol_fee_bps)?.safe_div(10_000)?;

  if is_wsol(&input_token_mint.key())? {
    let cpi_accounts = Transfer {
      from: ctx.accounts.input_token_account.to_account_info(),
      to: ctx.accounts.treasury_input_ata.to_account_info(),
      authority: ctx.accounts.payer.to_account_info(),
    };
  
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
  
    token::transfer(cpi_ctx, fees)?;
  } else {
    let cpi_accounts = TransferChecked {
      from: ctx.accounts.input_token_account.to_account_info(),
      mint: ctx.accounts.input_token_mint.to_account_info(),
      to: ctx.accounts.treasury_input_ata.to_account_info(),
      authority: ctx.accounts.payer.to_account_info(),
    };
  
    let cpi_program = ctx.accounts.token_2022.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
  
    token_2022::transfer_checked(cpi_ctx, fees, TOKEN_DECIMALS)?;
  }
 
  Ok(())
}

pub fn exec(ctx: Context<Swap>, max_amount_in: u64, amount_out_less_fee: u64) -> Result<()> {
  let input_token_balance_before = ctx.accounts.input_token_account.amount;
  swap(&ctx, max_amount_in, amount_out_less_fee)?;
  ctx.accounts.input_token_account.reload()?;
  let input_token_balance_after = ctx.accounts.input_token_account.amount;

  let token_amount_received = input_token_balance_after.safe_sub(input_token_balance_before)?;
  collect_fees(&ctx, token_amount_received)?;
  Ok(())
}
