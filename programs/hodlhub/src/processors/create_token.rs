use anchor_lang::{prelude::*, solana_program::{program::invoke, system_instruction::transfer}};
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

pub fn update_account_lamports_to_minimum_balance<'info>(
  account: AccountInfo<'info>,
  payer: AccountInfo<'info>,
  system_program: AccountInfo<'info>,
) -> Result<()> {
  let extra_lamports = Rent::get()?.minimum_balance(account.data_len()) - account.get_lamports();

  if extra_lamports > 0 {
    invoke(
      &transfer(payer.key, account.key, extra_lamports),
      &[payer, account, system_program],
    )?;
  }
  Ok(())
}

fn create_metadata(
  ctx: &Context<CreateToken>,
  name: String,
  symbol: String,
  uri: String,
) -> Result<()> {
  let token_key = &ctx.accounts.token.key();
  let state_key = &ctx.accounts.state.key();
  let seeds: &[&[u8]] = &[
    b"bonding_curve",
    state_key.as_ref(),
    token_key.as_ref(),
    &[ctx.bumps.bonding_curve],
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  let cpi_accounts = TokenMetadataInitialize {
    token_program_id: ctx.accounts.token_2022.to_account_info(),
    mint: ctx.accounts.token.to_account_info(),
    metadata: ctx.accounts.token.to_account_info(), // metadata account is the mint, since data is stored in mint
    mint_authority: ctx.accounts.bonding_curve.to_account_info(),
    update_authority: ctx.accounts.bonding_curve.to_account_info(),
  }
  ;
  let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_2022.to_account_info(), cpi_accounts, signer_seeds);
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
    ctx.accounts.token.key(),
    state.sol_target,
    state.protocol_fee,
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
