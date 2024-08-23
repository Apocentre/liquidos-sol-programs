use anchor_lang::prelude::*;
use anchor_spl::token::{set_authority, spl_token::instruction::AuthorityType, SetAuthority};
use crate::instructions::revoke_minting::RevokeMinting;

fn revoke_mint_authority(ctx: &Context<RevokeMinting>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let cpi_accounts = SetAuthority {
    current_authority: ctx.accounts.bonding_curve.to_account_info(),
    account_or_mint: ctx.accounts.token.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  set_authority(cpi_ctx, AuthorityType::MintTokens, None)
}

pub fn exec(ctx: Context<RevokeMinting>) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;
  
  if curve.closed == 1 {
    let state_key = &ctx.accounts.state.key();
    let token = &ctx.accounts.token.key();
    let seeds: &[&[u8]] = &[
      b"bonding_curve",
      state_key.as_ref(),
      token.as_ref(),
      &[curve.bump],
    ];
    let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

    revoke_mint_authority(&ctx, signer_seeds)?;
  }

  Ok(())
}
