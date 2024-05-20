use anchor_lang::{
  prelude::*,
  solana_program::{
    instruction::Instruction, program::{invoke, invoke_signed}, system_instruction::transfer
  }
};
use anchor_spl::token_2022::{MintTo, mint_to};
use ::borsh::BorshSerialize;
use crate::{
  instructions::buy::Buy, program_error::ErrorCode, raydium,
};

fn mint_tokens(ctx: &Context<Buy>, amount: u64, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let cpi_accounts = MintTo {
    mint: ctx.accounts.token.to_account_info(),
    to: ctx.accounts.buyer_ata.to_account_info(),
    authority: ctx.accounts.token_authority.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  mint_to(cpi_ctx, amount)?;

  Ok(())
}

fn accept_sol(ctx: &Context<Buy>, amount: u64) -> Result<()> {
  let buyer = &ctx.accounts.buyer;
  let bonding_curve = &ctx.accounts.bonding_curve;

  invoke(
    &transfer(&buyer.key(), &bonding_curve.key(), amount),
    &[
      buyer.to_account_info(),
      bonding_curve.to_account_info(),
    ],
  )?;

  Ok(())
}

// create a raydium pool with the current liquidity
fn move_liquidity(
  ctx: &Context<Buy>,
  init_amount_0: u64,
  init_amount_1: u64,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  let token_key = &ctx.accounts.token;
  let wsol_token_key = &ctx.accounts.wsol_token;

  // Raydium expect token_0 to be smaller that token_1
  let (
    token_0,
    token_1,
    creator_token_0,
    creator_token_1,
  ) = if token_key.key() < wsol_token_key.key() {
    (
      token_key.to_account_info(),
      wsol_token_key.to_account_info(),
      ctx.accounts.buyer_ata.to_account_info(),
      ctx.accounts.buyer_wsol_ata.to_account_info(),
    )
  } else {
    (
      wsol_token_key.to_account_info(),
      token_key.to_account_info(),
      ctx.accounts.buyer_wsol_ata.to_account_info(),
      ctx.accounts.buyer_ata.to_account_info(),
    )
  };

  let accounts = vec![
    AccountMeta::new(ctx.accounts.buyer.key(), true),
    AccountMeta::new_readonly(ctx.accounts.amm_config.key(), false),
    AccountMeta::new_readonly(ctx.accounts.raydium_authority.key(), false),
    AccountMeta::new(ctx.accounts.pool_state.key(), false),
    AccountMeta::new_readonly(token_0.key(), false),
    AccountMeta::new_readonly(token_1.key(), false),
    AccountMeta::new(ctx.accounts.lp_mint.key(), false),
    AccountMeta::new(creator_token_0.key(), false),
    AccountMeta::new(creator_token_1.key(), false),
    AccountMeta::new(ctx.accounts.creator_lp_token.key(), false),
    AccountMeta::new(ctx.accounts.token_0_vault.key(), false),
    AccountMeta::new(ctx.accounts.token_1_vault.key(), false),
    AccountMeta::new(ctx.accounts.create_pool_fee.key(), false),
    AccountMeta::new(ctx.accounts.observation_state.key(), false),
    AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
    AccountMeta::new_readonly(ctx.accounts.token_2022.key(), false),
    AccountMeta::new_readonly(ctx.accounts.token_2022.key(), false),
    AccountMeta::new_readonly(ctx.accounts.associated_token_program.key(), false),
    AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
    AccountMeta::new_readonly(ctx.accounts.rent.key(), false),
  ];

  let mut data: Vec<u8> = Vec::new();
  raydium::InitializeIx {
    init_amount_0,
    init_amount_1,
    open_time: 0,
  }.serialize(&mut data)?;

  let ix = Instruction {
    program_id: raydium::id(),
    accounts,
    data,
  };

  invoke_signed(
    &ix,
    &[
      ctx.accounts.buyer.to_account_info(),
      ctx.accounts.amm_config.to_account_info(),
      ctx.accounts.raydium_authority.to_account_info(),
      ctx.accounts.pool_state.to_account_info(),
      token_0,
      token_1,
      ctx.accounts.lp_mint.to_account_info(),
      creator_token_0,
      creator_token_1,
      ctx.accounts.creator_lp_token.to_account_info(),
      ctx.accounts.token_0_vault.to_account_info(),
      ctx.accounts.token_1_vault.to_account_info(),
      ctx.accounts.create_pool_fee.to_account_info(),
      ctx.accounts.observation_state.to_account_info(),
      ctx.accounts.token_program.to_account_info(),
      ctx.accounts.token_2022.to_account_info(),
      ctx.accounts.token_2022.to_account_info(),
      ctx.accounts.associated_token_program.to_account_info(),
      ctx.accounts.system_program.to_account_info(),
      ctx.accounts.rent.to_account_info(),
    ],
    signer_seeds,
  )?;
  
  Ok(())
}

pub fn exec(
  ctx: Context<Buy>,
  amount: u64,
  min_amount_out: u64,
) -> Result<()> {
  let curve = &mut ctx.accounts.bonding_curve;
  let spendable_amount = u64::min(curve.max_accepted_amount()?, amount);

  // Slippage check
  let token_amount = curve.calculate_purchase_return(spendable_amount)?;
  require!(token_amount > min_amount_out, ErrorCode::SlippageViolation);

  {
    let token = &ctx.accounts.token.key();
    let state_key = &ctx.accounts.state.key();
    let seeds: &[&[u8]] = &[
      b"token_authority",
      state_key.as_ref(),
      token.as_ref(),
      &[ctx.accounts.bonding_curve.token_authority_bump],
    ];
    let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

    let curve = &ctx.accounts.bonding_curve;
    mint_tokens(&ctx, token_amount, signer_seeds)?;
    accept_sol(&ctx, spendable_amount)?;

    if curve.is_complete() {
      // TODO: find the correct amount of WSOL and Tokens to be added
      move_liquidity(&ctx, 1, 1, signer_seeds)?;
    }
  }

  Ok(())
}
