use anchor_lang::prelude::*;
use anchor_spl::token_2022::{MintTo, mint_to};
use crate::{
  instructions::buy::Buy, program_error::ErrorCode,
};

fn mint_tokens(ctx: &Context<Buy>, amount: u64) -> Result<()> {
  let token = &ctx.accounts.token.key();
  let state_key = &ctx.accounts.state.key();
  let seeds: &[&[u8]] = &[
    b"token_authority",
    state_key.as_ref(),
    token.as_ref(),
    &[ctx.accounts.bonding_curve.token_authority_bump],
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];
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


pub fn exec(
  ctx: Context<Buy>,
  amount: u64,
  min_amount_out: u64,
) -> Result<()> {
  let curve = &mut ctx.accounts.bonding_curve;
  let spendable_amount = u64::min(curve.max_accepted_amount(), amount);

  if amount > spendable_amount {
    todo!("send amount - spendable_amount back to the user")  
  }

  // Slippgae check
  let token_amount = curve.calculate_purchase_return(spendable_amount)?;
  require!(token_amount > min_amount_out, ErrorCode::SlippageViolation);

  mint_tokens(&ctx, token_amount)?;

  Ok(())
}
