use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token_2022::{initialize_mint, InitializeMint}, token_interface::{
  metadata_pointer_initialize, token_metadata_initialize, transfer_fee_initialize,
  MetadataPointerInitialize, TokenMetadataInitialize, TransferFeeInitialize,
}};
use crate::{
  account_data::bonding_curve::BondingCurve,
  instructions::create_tax_token::CreateTaxToken, processors::create_token::TokenCreatedEvent,
};
use super::create_token::update_account_lamports_to_minimum_balance;

const DECIMALS: u8 = 6;

fn register_transfer_fee_extention(
  ctx: &Context<CreateTaxToken>,
  fee_bps: u16,
  max_fee: u64,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  let token = &ctx.accounts.token;
  let token_2022 = &ctx.accounts.token_2022;
  let cpi_accounts = TransferFeeInitialize {
    token_program_id: token_2022.to_account_info(),
    mint: token.to_account_info(),
  };
  let cpi_ctx = CpiContext::new_with_signer(token_2022.to_account_info(), cpi_accounts, signer_seeds);
  
  transfer_fee_initialize(
    cpi_ctx,
    Some(&ctx.accounts.bonding_curve.key()),
    Some(&ctx.accounts.token_creator.key()),
    fee_bps,
    max_fee
  )?;

  Ok(())
}

fn register_metadata_pointer_extention(ctx: &Context<CreateTaxToken>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let token = &ctx.accounts.token;
  let token_2022 = &ctx.accounts.token_2022;
  let cpi_accounts = MetadataPointerInitialize {
    token_program_id: token_2022.to_account_info(),
    mint: token.to_account_info()
  };
  let cpi_ctx = CpiContext::new_with_signer(token_2022.to_account_info(), cpi_accounts, signer_seeds);
  
  metadata_pointer_initialize(
    cpi_ctx,
    Some(ctx.accounts.bonding_curve.key()),
    Some(token.key()),
  )?;

  Ok(())
}

fn init_mint(ctx: &Context<CreateTaxToken>, signer_seeds: &[&[&[u8]]],) -> Result<()> {
  let token = &ctx.accounts.token;
  let token_2022 = &ctx.accounts.token_2022;
  let cpi_accounts = InitializeMint {
    mint: token.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
  };
  let cpi_ctx = CpiContext::new_with_signer(token_2022.to_account_info(), cpi_accounts, signer_seeds);
  initialize_mint(
    cpi_ctx,
    DECIMALS,
    &ctx.accounts.bonding_curve.key(),
    Some(&ctx.accounts.bonding_curve.key()),
  )?;

  Ok(())
}

fn init_metadata(
  ctx: &Context<CreateTaxToken>,
  name: String,
  symbol: String,
  uri: String,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  let token_2022 = &ctx.accounts.token_2022;
  let cpi_accounts = TokenMetadataInitialize {
    token_program_id: token_2022.to_account_info(),
    mint: ctx.accounts.token.to_account_info(),
    metadata: ctx.accounts.token.to_account_info(), // metadata account is the mint, since data is stored in mint
    mint_authority: ctx.accounts.bonding_curve.to_account_info(),
    update_authority: ctx.accounts.bonding_curve.to_account_info(),
  };
  
  let cpi_ctx = CpiContext::new_with_signer(token_2022.to_account_info(), cpi_accounts, signer_seeds);
  token_metadata_initialize(cpi_ctx, name, symbol, uri)?;

  // the new metadata will be stored on the Mint account. However, we have allocated enougg space for
  // the MetadataPoint and the Mint data. We need to allocate additional space to fit the metadata.
  update_account_lamports_to_minimum_balance(
    ctx.accounts.token.to_account_info(),
    ctx.accounts.token_creator.to_account_info(),
    ctx.accounts.system_program.to_account_info(),
  )?;
  
  Ok(())
}

fn setup_mint(
  ctx: &Context<CreateTaxToken>,
  name: String,
  symbol: String,
  uri: String,
  fee_bps: u16,
  max_fee: u64,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  register_transfer_fee_extention(ctx, fee_bps, max_fee, signer_seeds)?;
  register_metadata_pointer_extention(ctx, signer_seeds)?;
  init_mint(ctx, signer_seeds)?;
  init_metadata(ctx, name, symbol, uri, signer_seeds)?;
  
  Ok(())
}

fn create_curve_ata(ctx: &Context<CreateTaxToken>, signer_seeds: &[&[&[u8]]],) -> Result<()> {
  let associated_token_program = &ctx.accounts.associated_token_program;
  let cpi_accounts = associated_token::Create {
    payer: ctx.accounts.token_creator.to_account_info(),
    associated_token: ctx.accounts.curve_ata.to_account_info(),
    authority: ctx.accounts.bonding_curve.to_account_info(),
    mint: ctx.accounts.token.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    token_program: ctx.accounts.token_2022.to_account_info(),
  };
  let cpi_ctx = CpiContext::new_with_signer(associated_token_program.to_account_info(), cpi_accounts, signer_seeds);
  associated_token::create(cpi_ctx)?;
  
  Ok(())
}

pub fn exec(
  ctx: Context<CreateTaxToken>,
  name: String,
  symbol: String,
  uri: String,
  fee_bps: u16,
  max_fee: u64, 
) -> Result<()> {
  let state = &ctx.accounts.state;
  let token_creator = ctx.accounts.token_creator.key();
  let curve_key = ctx.accounts.bonding_curve.key();
  let curve = &mut ctx.accounts.bonding_curve;

  ***curve = BondingCurve::new(
    token_creator,
    ctx.accounts.token.key(),
    state.sol_target,
    state.protocol_fee,
    state.trade_fee_bps,
    state.creator_fee,
    state.total_token_supply,
    ctx.bumps.bonding_curve,
  );

  let token_key = &ctx.accounts.token.key();
  let state_key = &ctx.accounts.state.key();
  let seeds: &[&[u8]] = &[
    b"bonding_curve",
    state_key.as_ref(),
    token_key.as_ref(),
    &[ctx.bumps.bonding_curve],
  ];
  let signer_seeds: &[&[&[u8]]] = &[&seeds[..]];
  setup_mint(&ctx, name.clone(), symbol.clone(), uri.clone(), fee_bps, max_fee, signer_seeds)?;
  create_curve_ata(&ctx, signer_seeds)?;

  emit!(TokenCreatedEvent {
    creator: token_creator,
    address: ctx.accounts.token.key(),
    name,
    symbol,
    uri,
    curve: curve_key,
    has_tax: true,
  });

  Ok(())
}
