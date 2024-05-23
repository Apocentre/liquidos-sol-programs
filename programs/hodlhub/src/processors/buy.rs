use anchor_lang::{
  prelude::*,
  solana_program::{
    instruction::Instruction, program::{invoke, invoke_signed}, system_instruction::transfer
  },
};
use anchor_spl::{token::{burn, sync_native, Burn, SyncNative}, token_2022::{mint_to, MintTo}};
use ::borsh::BorshSerialize;
use crate::{
  instructions::buy::Buy, processors::common::transfer_from_pda, program_error::ErrorCode, raydium
};

#[event]
pub struct BuyEvent {
  buyer: Pubkey,
  token: Pubkey,
  sol_amount: u64,
  token_amount: u64,
  is_complete: bool,
}

fn mint_tokens(
  ctx: &Context<Buy>,
  amount: u64,
  signer_seeds: &[&[&[u8]]]
) -> Result<()> {
  let cpi_accounts = MintTo {
    mint: ctx.accounts.token.to_account_info(),
    to: ctx.accounts.buyer_ata.to_account_info(),
    authority: ctx.accounts.bonding_curve.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  mint_to(cpi_ctx, amount)?;

  Ok(())
}

fn send_sol_to_curve(ctx: &Context<Buy>, amount: u64) -> Result<()> {
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
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;
  let token_key = &ctx.accounts.token;
  let wsol_token_key = &ctx.accounts.wsol_token;
  let token_liquidity = curve.calculate_token_amount_to_mint()?;

  // Raydium expect token_0 to be smaller that token_1
  let (
    token_0,
    token_1,
    creator_token_0,
    creator_token_1,
    init_amount_0,
    init_amount_1,
  ) = if token_key.key() < wsol_token_key.key() {
    (
      token_key.to_account_info(),
      wsol_token_key.to_account_info(),
      ctx.accounts.buyer_ata.to_account_info(),
      ctx.accounts.buyer_wsol_ata.to_account_info(),
      token_liquidity,
      curve.reserve_token_balance,
    )
  } else {
    (
      wsol_token_key.to_account_info(),
      token_key.to_account_info(),
      ctx.accounts.buyer_wsol_ata.to_account_info(),
      ctx.accounts.buyer_ata.to_account_info(),
      curve.reserve_token_balance,
      token_liquidity,
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

/// Send WSOL and TOKKEN to the buyer whose purchase triggered the liquidity move.
/// This buyers is the creator of the Raydium pool so it has to have the funds to do so.
fn fund_creator_account(ctx: &Context<Buy>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;

  // 1. mint curve.calculate_token_amount_to_mint() tokens to the buyer_ata
  let token_liquidity = curve.calculate_token_amount_to_mint()?;
  mint_tokens(&ctx, token_liquidity, signer_seeds)?;

  // 2. convert SOL from the curve into WSOL and send to buyer
  let mut buyer_wsol_ata = ctx.accounts.buyer_wsol_ata.to_account_info();
  transfer_from_pda(
    &mut ctx.accounts.bonding_curve.to_account_info(),
    &mut buyer_wsol_ata,
    curve.sol_target,
  )?;

  let cpi_accounts = SyncNative {
    account: buyer_wsol_ata,
  };
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
  sync_native(cpi_ctx)?;

  todo!()
}

/// Burns the LP created in the move_liquidity. These LP tokens are sent to the buyer
/// whose purchase triggered the liquidity move. We need to burn this liquidity
fn burn_lp(ctx: &mut Context<Buy>) -> Result<()> {
  let creator_lp_token = &mut ctx.accounts.creator_lp_token;
  let cpi_accounts = Burn {
    mint: ctx.accounts.lp_mint.to_account_info(),
    from: creator_lp_token.to_account_info(),
    authority: ctx.accounts.buyer.to_account_info(),
  };
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
  
  // reload the ata and check the new balance
  creator_lp_token.reload()?;
  let lp_balance = creator_lp_token.amount;
  
  burn(cpi_ctx, lp_balance)
}

pub fn exec(
  mut ctx: Context<Buy>,
  amount: u64,
  min_amount_out: u64,
) -> Result<()> {
  let curve = &mut ctx.accounts.bonding_curve;
  let spendable_amount = u64::min(curve.max_accepted_amount()?, amount);

  // Slippage check
  let token_amount = curve.calculate_purchase_return(spendable_amount)?;
  require!(token_amount > min_amount_out, ErrorCode::SlippageViolation);

  let token = &ctx.accounts.token.key();
  let state_key = &ctx.accounts.state.key();
  let seeds: &[&[u8]] = &[
    b"bonding_curve",
    state_key.as_ref(),
    token.as_ref(),
    &[ctx.accounts.bonding_curve.bump],
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  let curve = &ctx.accounts.bonding_curve;
  mint_tokens(&ctx, token_amount, signer_seeds)?;
  send_sol_to_curve(&ctx, spendable_amount)?;

  if curve.is_complete() {
    fund_creator_account(&ctx, signer_seeds)?;
    move_liquidity(&ctx, signer_seeds)?;
    burn_lp(&mut ctx)?;
  }

  {
    let curve = &ctx.accounts.bonding_curve;

    emit!(BuyEvent {
      buyer: ctx.accounts.buyer.key(),
      token: ctx.accounts.token.key(),
      sol_amount: spendable_amount,
      token_amount,
      is_complete: curve.is_complete(),
    });
  }


  Ok(())
}
