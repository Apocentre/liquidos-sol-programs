use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::{
  token_2022, token_2022_extensions::spl_token_metadata_interface::instruction::initialize as initialize_metadata,
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
) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve.key();
  let token_key = &ctx.accounts.token.key();
  let state_key = &ctx.accounts.state.key();
  let seeds: &[&[u8]] = &[
    b"bonding_curve",
    state_key.as_ref(),
    token_key.as_ref(),
    &[ctx.bumps.bonding_curve],
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];
  
  // Initialize the metadata account
  let init_metadata_ix = initialize_metadata(
    &token_2022::ID,
    &token_key,
    curve,
    &token_key,
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
