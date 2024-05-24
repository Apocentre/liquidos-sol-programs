use anchor_lang::{
  prelude::*,
  solana_program::{
    program::invoke, system_instruction::transfer
  },
};
use anchor_spl::token_2022::{mint_to, MintTo};
use crate::{
  instructions::buy::Buy, processors::common::transfer_from_pda, program_error::ErrorCode,
};

#[event]
pub struct BuyEvent {
  buyer: Pubkey,
  token: Pubkey,
  sol_amount: u64,
  token_amount: u64,
  is_complete: bool,
  price: u64,
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
  let curve = &ctx.accounts.bonding_curve;

  invoke(
    &transfer(&buyer.key(), &curve.key(), amount),
    &[
      buyer.to_account_info(),
      curve.to_account_info(),
    ],
  )?;

  Ok(())
}

/// Collects fees from the SOL accumulated in the pool
fn collect_fees(ctx: &Context<Buy>) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve.load()?;

  transfer_from_pda(
    &mut ctx.accounts.bonding_curve.to_account_info(),
    &mut ctx.accounts.treasury.to_account_info(),
    curve.calc_protocol_fees()?,
  )?;

  Ok(())
}

/// Collects trade fees on each transaction. Fees collected in SOL
fn collect_trade_fees(ctx: &Context<Buy>, sol_amount: u64) -> Result<()> {
  let buyer = &ctx.accounts.buyer;
  let curve = &ctx.accounts.bonding_curve.load()?;
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

pub fn exec(
  ctx: Context<Buy>,
  amount: u64,
  min_amount_out: u64,
) -> Result<()> {
  let curve = &mut ctx.accounts.bonding_curve.load_mut()?;
  require!(curve.closed == 0, ErrorCode::CurveClosed);
  let spendable_amount = u64::min(curve.max_accepted_amount()?, amount);

  // Slippage check
  let token_amount = curve.process_purchase_return(spendable_amount)?;
  require!(token_amount >= min_amount_out, ErrorCode::SlippageViolation);
  let price = curve.price;

  let token = &ctx.accounts.token.key();
  let state_key = &ctx.accounts.state.key();
  let seeds: &[&[u8]] = &[
    b"bonding_curve",
    state_key.as_ref(),
    token.as_ref(),
    &[curve.bump],
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  collect_trade_fees(&ctx, spendable_amount)?;
  mint_tokens(&ctx, token_amount, signer_seeds)?;
  send_sol_to_curve(&ctx, spendable_amount)?;

  if curve.is_complete() {
    collect_fees(&ctx)?;
    
    // mark the curve as closed
    let curve = &mut ctx.accounts.bonding_curve.load_mut()?;
    curve.close_curve();
  }

  {
    let buyer = ctx.accounts.buyer.key();

    emit!(BuyEvent {
      buyer,
      token: *token,
      sol_amount: spendable_amount,
      token_amount,
      is_complete: curve.is_complete(),
      price,
    });
  }


  Ok(())
}
