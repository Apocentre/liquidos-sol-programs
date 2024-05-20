use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::{
  token_2022,
  token_2022_extensions::{
    metadata_pointer::{MetadataPointerInitialize, metadata_pointer_initialize},
    spl_token_metadata_interface::instruction::initialize as initialize_metadata,
  },
};
use crate::{
  account_data::bonding_curve::BondingCurve,
  instructions::create_token::CreateToken,
};

fn create_metadata(
  ctx: &Context<CreateToken>,
  name: String,
  symbol: String,
  uri: String,
) -> Result<()> {
  let token = &ctx.accounts.token.key();
  let state_key = &ctx.accounts.state.key();
  let seeds: &[&[u8]] = &[
    b"token_authority",
    state_key.as_ref(),
    token.as_ref(),
    &[ctx.bumps.token_authority],
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];
  let token_authority = &ctx.accounts.token_authority.key();

  // Initialize the metadata pointer first
  let cpi_metadata_pointer_init_accounts = MetadataPointerInitialize {
    token_program_id: ctx.accounts.token_2022.to_account_info(),
    mint: ctx.accounts.token.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_metadata_pointer_init_accounts, signer_seeds);
  metadata_pointer_initialize(cpi_ctx, Some(token_authority.clone()), Some(token.clone()))?;

  // Initialize the metadata account
  let init_metadata_ix = initialize_metadata(
    &token_2022::ID,
    &ctx.accounts.token.key(),
    token_authority,
    token,
    token_authority,
    name,
    symbol,
    uri,
  );

  let token_acc_info = ctx.accounts.token.to_account_info();
  let token_authority_acc_info = ctx.accounts.token_authority.to_account_info();
  invoke_signed(
    &init_metadata_ix,
    &[
      token_acc_info.clone(),
      token_authority_acc_info.clone(),
      token_acc_info,
      token_authority_acc_info,
    ],
    signer_seeds,
  )?;

  Ok(())
}

pub fn exec(
  ctx: Context<CreateToken>,
  name: String,
  symbol: String,
  uri: String,
) -> Result<()> {
  let token_creator = ctx.accounts.token_creator.key();
  let curve = &mut ctx.accounts.bonding_curve;
  **curve = BondingCurve::new(token_creator, ctx.bumps.bonding_curve);

  create_metadata(&ctx, name, symbol, uri)?;

  Ok(())
}
