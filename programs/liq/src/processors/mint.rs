use anchor_lang::prelude::*;
use anchor_spl::token_2022::{mint_to, MintTo};
use crate::instructions::mint::Mint;

pub fn exec(
  ctx: Context<Mint>,
  amount: u64,
) -> Result<()> {
  // TODO: apply the curve logic
  Ok(())
}

fn mint_tokens(
  ctx: &Context<Mint>,
  amount: u64,
  signer_seeds: &[&[&[u8]]]
) -> Result<()> {
  let cpi_accounts = MintTo {
    mint: ctx.accounts.liq_token.to_account_info(),
    to: ctx.accounts.buyer_liq_ata.to_account_info(),
    authority: ctx.accounts.bonding_curve.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  mint_to(cpi_ctx, amount)?;

  Ok(())
}
