use anchor_lang::prelude::*;
use crate::instructions::mint_liq::MintLiq;

pub fn exec<'info>(ctx: Context<MintLiq>, amount: u64) -> Result<()> {
  mint_liq_cpi(ctx, amount)?;

  Ok(())
}

fn mint_liq_cpi<'info>(ctx: Context<MintLiq>, amount: u64) -> Result<()> {
  let cpi_program = ctx.accounts.liq_program.to_account_info();
  let cpi_accounts = liq::cpi::accounts::Mint {
    state: ctx.accounts.liq_state.clone(),
    buyer: ctx.accounts.buyer.to_account_info(),
    bonding_curve: ctx.accounts.liq_bonding_curve.clone(),
    liq_token: ctx.accounts.liq_token.clone(),
    buyer_liq_ata: ctx.accounts.buyer_liq_ata.clone(),
    curve_creator: ctx.accounts.curve_creator.clone(),
    curve_creator_liq_ata: ctx.accounts.curve_creator_liq_ata.clone(),
    source_bonding_curve: ctx.accounts.bonding_curve.to_account_info(),
    token_2022: ctx.accounts.token_2022.to_account_info(),
    associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    event_authority: ctx.accounts.liq_event_authority.clone(),
    program: ctx.accounts.liq_program.to_account_info(),
  };

  let bonding_curve = &ctx.accounts.bonding_curve;
  let state_key = ctx.accounts.state.key();
  let token_key = ctx.accounts.token.key();
  let seeds: &[&[u8]] = &[
    b"bonding_curve", state_key.as_ref(), token_key.as_ref(),
    &[bonding_curve.bump]
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  liq::cpi::mint(cpi_ctx, token_key, amount)?;

  Ok(())
}
