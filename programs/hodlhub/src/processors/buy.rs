use anchor_lang::{
  prelude::*,
  solana_program::{
    program::invoke, system_instruction::transfer, sysvar::instructions::{load_current_index_checked, load_instruction_at_checked}
  }, Discriminator,
};
use anchor_spl::token_2022::{mint_to, MintTo};
use crate::{
  instruction::MoveLiquidity, instructions::buy::Buy, processors::common::transfer_from_pda, program_error::ErrorCode, ID
};

#[event]
pub struct BuyEvent {
  buyer: Pubkey,
  token: Pubkey,
  sol_amount: u64,
  token_amount: u64,
  is_complete: bool,
  price: u64,
  total_supply: u64,
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

fn send_sol_to_curve<'info>(
  ctx: &Context<'_, '_, '_, 'info, Buy<'info>>,
  amount: u64,
  curve_key: Pubkey,
  curve_acc_info: AccountInfo<'info>,
) -> Result<()> {
  let buyer = &ctx.accounts.buyer;

  invoke(
    &transfer(&buyer.key(), &curve_key, amount),
    &[
      buyer.to_account_info(),
      curve_acc_info,
    ],
  )?;

  Ok(())
}

/// Collects fees from the SOL accumulated in the pool
fn collect_fees(ctx: &Context<Buy>, mut curve_acc_info: AccountInfo<'_>) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;

  transfer_from_pda(
    &mut curve_acc_info,
    &mut ctx.accounts.treasury.to_account_info(),
    curve.calc_protocol_fees()?,
  )?;

  Ok(())
}

/// Collects trade fees on each transaction. Fees collected in SOL
fn collect_trade_fees(ctx: &Context<Buy>, sol_amount: u64) -> Result<()> {
  let buyer = &ctx.accounts.buyer;
  let curve = &ctx.accounts.bonding_curve;
  let trade_fees = curve.calc_trade_fees(sol_amount)?;
  let treasury = &ctx.accounts.treasury;

  invoke(
    &transfer(&buyer.key(), &treasury.key(), trade_fees),
    &[
      buyer.to_account_info(),
      treasury.to_account_info(),
    ],
  )?;

  Ok(())
}

/// Send WSOL and TOKKEN to the buyer whose purchase triggered the liquidity move.
/// This buyers is the creator of the Raydium pool so it has to have the funds to do so.
fn fund_creator_account(ctx: &Context<Buy>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;

  // 1. mint curve.calculate_token_amount_to_mint() tokens to the buyer_ata
  let token_liquidity = curve.calc_token_amount_to_mint()?;
  mint_tokens(&ctx, token_liquidity, signer_seeds)?;

  // 2. Send SOL to the buyer's WSOL ATA which will later be synced i.e. converted into WSOL
  let mut buyer_wsol_ata = ctx.accounts.buyer_wsol_ata.to_account_info();
  transfer_from_pda(
    &mut ctx.accounts.bonding_curve.to_account_info(),
    &mut buyer_wsol_ata,
    curve.net_reserve_token_liquidity()?,
  )?;

  Ok(())
}

/// When buying a token, the buyer will send two ixs: the Buy and the MoveLiquidity.
/// The later will be ignored in the move_liquidity processor is the curve is not closed.
/// This is important so we know that once the SOL is sent to the buyer_wsol_ata he atomically
/// moves_liquidity.
fn instrospect_next_ix(ctx: &Context<Buy>) -> Result<()> {
  let current_index = load_current_index_checked(&ctx.accounts.ix_sysvar.to_account_info())?;
  let current_ix = load_instruction_at_checked((current_index + 1) as usize, &ctx.accounts.ix_sysvar.to_account_info())?;
  require!(current_ix.program_id.eq(&ID), ErrorCode::WrongProgramId);
  
  let discriminator: [u8; 8] = current_ix.data[..8].try_into().map_err(|_| ErrorCode::WrongIxData)?;
  require!(discriminator.eq(&MoveLiquidity::DISCRIMINATOR), ErrorCode::ExpectedMoveLiquidityIx);
  
  Ok(())
}

pub fn exec<'info>(
  ctx: Context<'_, '_, '_, 'info, Buy<'info>>,
  amount: u64,
  min_amount_out: u64,
) -> Result<()> {
  let curve_key = ctx.accounts.bonding_curve.key();
  let curve_acc_info = ctx.accounts.bonding_curve.to_account_info();
  let curve = &mut ctx.accounts.bonding_curve;
  require!(curve.closed == 0, ErrorCode::CurveClosed);
  let spendable_amount = u64::min(curve.max_accepted_amount()?, amount);

  // Slippage check
  let token_amount = curve.process_purchase_return(spendable_amount)?;
  require!(token_amount >= min_amount_out, ErrorCode::SlippageViolation);

  let token = &ctx.accounts.token.key();
  let state_key = &ctx.accounts.state.key();
  let seeds: &[&[u8]] = &[
    b"bonding_curve",
    state_key.as_ref(),
    token.as_ref(),
    &[curve.bump],
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  let curve = &ctx.accounts.bonding_curve;
  collect_trade_fees(&ctx, spendable_amount)?;
  mint_tokens(&ctx, token_amount, signer_seeds)?;
  send_sol_to_curve(&ctx, spendable_amount, curve_key, curve_acc_info.clone())?;

  let price = curve.price;
  let total_supply = curve.total_supply;
  let is_complete = curve.is_complete();

  if is_complete {
    fund_creator_account(&ctx, signer_seeds)?;
    collect_fees(&ctx, curve_acc_info)?;
    
    // mark the curve as closed
    let curve = &mut ctx.accounts.bonding_curve;
    curve.close_curve();

    instrospect_next_ix(&ctx)?;
  }

  {
    let buyer = ctx.accounts.buyer.key();

    emit!(BuyEvent {
      buyer,
      token: *token,
      sol_amount: spendable_amount,
      token_amount,
      is_complete,
      price,
      total_supply,
    });
  }

  Ok(())
}
