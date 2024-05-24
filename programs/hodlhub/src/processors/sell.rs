use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use anchor_spl::token_2022::{burn, Burn};
use super::common::transfer_from_pda;
use crate::{
  program_error::ErrorCode, instructions::sell::Sell,
};

#[event]
pub struct SellEvent {
  seller: Pubkey,
  token: Pubkey,
  token_amount: u64,
  sol_amount: u64,
  price: u64,
}

fn send_sol_to_seller(ctx: &Context<Sell>, amount: u64) -> Result<()> {
  transfer_from_pda(
    &mut ctx.accounts.bonding_curve.to_account_info(),
    &mut ctx.accounts.seller.to_account_info(),
    amount,
  )?;
  Ok(())
}

fn burn_tokens(ctx: &Context<Sell>, amount: u64) -> Result<()> {
  let seller_ata = &ctx.accounts.seller_ata;
  let cpi_accounts = Burn {
    mint: ctx.accounts.token.to_account_info(),
    from: seller_ata.to_account_info(),
    authority: ctx.accounts.seller.to_account_info(),
  };
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
  
  burn(cpi_ctx, amount)
}

/// Collects trade fees on each transaction. Fees collected in SOL
fn collect_trade_fees(ctx: &Context<Sell>, fees: u64) -> Result<()> {
  transfer_from_pda(
    &mut ctx.accounts.bonding_curve.to_account_info(),
    &mut ctx.accounts.treasury.to_account_info(),
    fees,
  )?;

  Ok(())
}

pub fn exec(
  ctx: Context<Sell>,
  token_amount: u64,
  min_sol_amount_out: u64,
) -> Result<()> {
  let curve = &mut ctx.accounts.bonding_curve.load_mut()?;
  require!(curve.closed == 0, ErrorCode::CurveClosed);
  let sol_amount = curve.process_sale_return(token_amount)?;
  require!(sol_amount >= min_sol_amount_out, ErrorCode::SlippageViolation);

  let price = curve.price;
  let fees = curve.calc_trade_fees(sol_amount)?;
  collect_trade_fees(&ctx, fees)?;

  let net_amount = sol_amount.safe_sub(fees)?;
  send_sol_to_seller(&ctx, net_amount)?;
  burn_tokens(&ctx, token_amount)?;

  emit!(SellEvent {
    seller: ctx.accounts.seller.key(),
    token: ctx.accounts.token.key(),
    sol_amount,
    token_amount,
    price,
  });

  Ok(())
}
