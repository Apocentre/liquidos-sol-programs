use anchor_lang::prelude::*;
use anchor_spl::token_2022::{burn, Burn};
use super::common::transfer_from_pda;
use crate::{
  program_error::ErrorCode, instructions::sell::Sell,
};

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


pub fn exec(
  ctx: Context<Sell>,
  token_amount: u64,
  min_sol_amount_out: u64,
) -> Result<()> {
  let curve = &mut ctx.accounts.bonding_curve;
  let sol_amount = curve.calculate_sale_return(token_amount)?;
  require!(sol_amount > min_sol_amount_out, ErrorCode::SlippageViolation);

  send_sol_to_seller(&ctx, sol_amount)?;
  burn_tokens(&ctx, token_amount)?;
  Ok(())
}
