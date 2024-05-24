use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::{
  token_2022::{self, InitializeMint2, initialize_mint2},
  associated_token::{create, Create},
  token_2022_extensions::{
    metadata_pointer::{metadata_pointer_initialize, MetadataPointerInitialize},
    spl_token_metadata_interface::instruction::initialize as initialize_metadata,
  }
};
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
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  let token = &ctx.accounts.token;
  let curve = &ctx.accounts.bonding_curve.key();

  // Initialize the metadata pointer first
  let cpi_metadata_pointer_init_accounts = MetadataPointerInitialize {
    token_program_id: ctx.accounts.token_2022.to_account_info(),
    mint: ctx.accounts.token.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_metadata_pointer_init_accounts, signer_seeds);
  let token_creator = &ctx.accounts.token_creator;

  metadata_pointer_initialize(
    cpi_ctx,
    Some(token_creator.key()),
    Some(token.key())
  )?;

  // Initialize the mint account
  let init_mint_accounts = InitializeMint2 {
    mint: token.to_account_info(),
  };
  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, init_mint_accounts, signer_seeds);
  initialize_mint2(cpi_ctx, 6, curve, None)?;

  // Initialize the metadata account
  let init_metadata_ix = initialize_metadata(
    &token_2022::ID,
    &token.key(),
    curve,
    &token.key(),
    curve,
    name,
    symbol,
    uri,
  );

  let token_acc_info = ctx.accounts.token.to_account_info();
  let curve_acc_info = ctx.accounts.bonding_curve.to_account_info();
  invoke_signed(
    &init_metadata_ix,
    &[
      token_acc_info.clone(),
      curve_acc_info.clone(),
      token_acc_info,
      curve_acc_info,
    ],
    signer_seeds,
  )?;

  Ok(())
}

fn create_curve_ata(ctx: &Context<CreateToken>, signer_seeds: &[&[&[u8]]],) -> Result<()> {
  let cpi_program = &ctx.accounts.associated_token_program;
  let cpi_accounts = Create {
    payer: ctx.accounts.token_creator.to_account_info(),
    associated_token: ctx.accounts.curve_ata.to_account_info(),
    authority: ctx.accounts.bonding_curve.to_account_info(),
    mint: ctx.accounts.token.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    token_program: ctx.accounts.token_2022.to_account_info(),
  };
  let cpi_ctx = CpiContext::new_with_signer(cpi_program.to_account_info(), cpi_accounts, signer_seeds);
  create(cpi_ctx)?;
  
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

  let token_key = &ctx.accounts.token.key();
  let state_key = &ctx.accounts.state.key();
  let seeds: &[&[u8]] = &[
    b"bonding_curve",
    state_key.as_ref(),
    token_key.as_ref(),
    &[ctx.bumps.bonding_curve],
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];
    
  create_metadata(&ctx, name.clone(), symbol.clone(), uri.clone(), signer_seeds)?;
  create_curve_ata(&ctx, signer_seeds)?;

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
