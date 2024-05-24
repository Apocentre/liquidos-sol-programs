use anchor_lang::prelude::*;
use anchor_spl::token_interface::{token_metadata_initialize, TokenMetadataInitialize};
use crate::{
  account_data::bonding_curve::BondingCurve,
  instructions::create_token::CreateToken,
};

#[event]
pub struct TokenCreatedEvent {
  creator: Pubkey,
  address: Pubkey,
  name: String,
  symbol: String,
  uri: String,
  curve: Pubkey,
}

fn create_metadata(
  ctx: &Context<CreateToken>,
  name: String,
  symbol: String,
  uri: String,
) -> Result<()> {
  let cpi_accounts = TokenMetadataInitialize {
    token_program_id: ctx.accounts.token_2022.to_account_info(),
    mint: ctx.accounts.token.to_account_info(),
    metadata: ctx.accounts.token.to_account_info(), // metadata account is the mint, since data is stored in mint
    mint_authority: ctx.accounts.bonding_curve.to_account_info(),
    update_authority: ctx.accounts.bonding_curve.to_account_info(),
};
  let cpi_ctx = CpiContext::new(ctx.accounts.token_2022.to_account_info(), cpi_accounts);
  token_metadata_initialize(cpi_ctx, name, symbol, uri)?;

  Ok(())
}

pub fn exec(
  ctx: Context<CreateToken>,
  name: String,
  symbol: String,
  uri: String,
) -> Result<()> {
  let state = &ctx.accounts.state;
  let token_creator = ctx.accounts.token_creator.key();
  let curve_key = ctx.accounts.bonding_curve.key();
  let curve = &mut ctx.accounts.bonding_curve;
  ***curve = BondingCurve::new(
    token_creator,
    state.sol_target,
    state.protocol_fee_bps,
    state.trade_fee_bps,
    ctx.bumps.bonding_curve,
  );

  create_metadata(&ctx, name.clone(), symbol.clone(), uri.clone())?;

  emit!(TokenCreatedEvent {
    creator: token_creator,
    address: ctx.accounts.token.key(),
    name,
    symbol,
    uri,
    curve: curve_key,
  });

  Ok(())
}
