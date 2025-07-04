use anchor_lang::{
  prelude::{borsh::BorshSerialize, *},
  solana_program::{
    program::invoke, system_instruction::transfer,
    sysvar::instructions::{load_current_index_checked, load_instruction_at_checked},
  }, Discriminator,
};
use anchor_safe_math::SafeMath;
use anchor_spl::token_2022::{mint_to, MintTo};
use crate::{
  instruction::MoveLiquidity, instructions::buy::Buy,
  processors::common::transfer_from_pda,
  program_error::ErrorCode, raydium::{self, AmmConfig}, ID,
};

use super::common::deser;

#[event]
pub struct BuyEvent {
  curve_type: u8,
  buyer: Pubkey,
  token: Pubkey,
  sol_amount: String,
  token_amount: String,
  is_complete: bool,
  price: String,
  circulating_supply: String,
  sol_balance: String,
  buyer_balance: String,
}

fn mint_tokens(
  ctx: &Context<Buy>,
  amount: u64,
  signer_seeds: &[&[&[u8]]]
) -> Result<()> {
  let cpi_accounts = MintTo {
    mint: ctx.accounts.token.to_account_info(),
    to: ctx.accounts.buyer_ata.to_account_info(),
    authority: ctx.accounts.bonding_curve.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  mint_to(cpi_ctx, amount)?;

  Ok(())
}

fn send_sol_to_curve<'info>(
  ctx: &Context<'_, '_, '_, 'info, Buy<'info>>,
  amount: u64,
  curve_key: Pubkey,
  curve_acc_info: AccountInfo<'info>,
) -> Result<()> {
  let buyer = &ctx.accounts.buyer;

  invoke(
    &transfer(&buyer.key(), &curve_key, amount),
    &[
      buyer.to_account_info(),
      curve_acc_info,
    ],
  )?;

  Ok(())
}

/// Collects fees from the SOL accumulated in the pool and sends to the treasury
fn collect_protocol_fees(ctx: &Context<Buy>, curve_acc_info: &AccountInfo<'_>) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;

  transfer_from_pda(
    &mut curve_acc_info.to_account_info(),
    &mut ctx.accounts.treasury.to_account_info(),
    curve.protocol_fee,
  )?;

  Ok(())
}

/// Collects fees from the SOL accumulated in the pool and sends to the bonding curve creator acount
fn collect_creator_fees(ctx: &Context<Buy>, mut curve_acc_info: AccountInfo<'_>) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;

  transfer_from_pda(
    &mut curve_acc_info,
    &mut ctx.accounts.token_creator.to_account_info(),
    curve.creator_fee,
  )?;

  Ok(())
}

/// Collects trade fees on each transaction. Fees collected in SOL
fn collect_trade_fees(ctx: &Context<Buy>, trade_fees: u64) -> Result<()> {
  let buyer = &ctx.accounts.buyer;
  let treasury = &ctx.accounts.treasury;

  invoke(
    &transfer(&buyer.key(), &treasury.key(), trade_fees),
    &[
      buyer.to_account_info(),
      treasury.to_account_info(),
    ],
  )?;

  Ok(())
}

/// Send WSOL and $TOKEN to the buyer whose purchase triggered the liquidity move.
/// This buyers is the creator of the Raydium pool so it has to have the funds to do so.
fn fund_creator_account(ctx: &Context<Buy>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;

  // 1. mint curve.calculate_token_amount_to_mint() tokens to the buyer_ata
  let token_liquidity = curve.calc_token_amount_to_mint()?;
  mint_tokens(&ctx, token_liquidity, signer_seeds)?;

  // 2. Transfer SOL to cover the Raydium creation pool fee. This amount is deducted
  // from the SOL balance of the pool. So the total liquidity that will be moved (WSOL)
  // will be less this amount.
  let amm_config: AmmConfig = deser(ctx.accounts.amm_config.clone())?;
  let total_amm_cost = amm_config.create_pool_fee.safe_add(raydium::RENT_COST)?;

  transfer_from_pda(
    &mut ctx.accounts.bonding_curve.to_account_info(),
    &mut ctx.accounts.buyer.to_account_info(),
    total_amm_cost,
  )?;

  // 3. Send SOL to the buyer's WSOL ATA which will later be synced i.e. converted into WSOL
  let mut buyer_wsol_ata = ctx.accounts.buyer_wsol_ata.to_account_info();
  let total_sol_liquidity = curve.net_reserve_token_liquidity()?.safe_sub(total_amm_cost)?;

  transfer_from_pda(
    &mut ctx.accounts.bonding_curve.to_account_info(),
    &mut buyer_wsol_ata,
    total_sol_liquidity,
  )?;

  Ok(())
}

/// When buying a token, the buyer will send two ixs: the Buy, and MoveLiquidity
/// The later will be executed but do nothing if the curve is not closed.
/// This is important so we know that once the SOL is sent to the buyer_wsol_ata he atomically
/// moves_liquidity
fn instrospect_next_ix(ctx: &Context<Buy>) -> Result<()> {
  let current_index = load_current_index_checked(&ctx.accounts.ix_sysvar.to_account_info())?;

  // check MoveLiquidity
  let current_ix = load_instruction_at_checked((current_index + 1) as usize, &ctx.accounts.ix_sysvar.to_account_info())?;
  require!(current_ix.program_id.eq(&ID), ErrorCode::WrongProgramId);
  let discriminator: [u8; 8] = current_ix.data[..8].try_into().map_err(|_| ErrorCode::WrongIxData)?;
  require!(discriminator.eq(&MoveLiquidity::DISCRIMINATOR), ErrorCode::ExpectedMoveLiquidityIx);

  Ok(())
}

pub fn exec<'info>(
  ctx: Context<'_, '_, '_, 'info, Buy<'info>>,
  amount: u64,
  min_amount_out: u64,
) -> Result<()> {
  let curve_key = ctx.accounts.bonding_curve.key();
  let curve_acc_info = ctx.accounts.bonding_curve.to_account_info();
  let curve = &mut ctx.accounts.bonding_curve;
  require!(curve.closed == 0, ErrorCode::CurveClosed);
  
  // calculate curve.max_accepted_amount() / 0.9 if given fees is 10%.
  // This way we make sure we accept as many SOL as needed to fill the curve and not less.
  // For example:
  // Current SOL in curve is 80 and user sends 15. The max_accepted_amount will be 10.8888888889
  // the trader fees 1.08888888889 (given a 10% trader fee) and thus the net_amount will be 9.8
  // which is exactly as much is needed to fill a curve v1 that accepts 89.8 max SOL.
  let max_accepted_amount = curve.max_accepted_amount()?
  .safe_mul(10_000)?
  .safe_div(10_000 - curve.trade_fee_bps)?;

  let spendable_amount = u64::min(max_accepted_amount, amount);
  let trade_fees = curve.calc_trade_fees(spendable_amount)?;
  let net_amount = spendable_amount.safe_sub(trade_fees)?;

  // Slippage check
  let token_amount = curve.process_purchase_return(net_amount)?;
  require!(token_amount >= min_amount_out, ErrorCode::SlippageViolation);

  let token = &ctx.accounts.token.key();
  let state_key = &ctx.accounts.state.key();
  let seeds: &[&[u8]] = &[
    b"bonding_curve",
    state_key.as_ref(),
    token.as_ref(),
    &[curve.bump],
  ];
  let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

  let curve = &ctx.accounts.bonding_curve;
  let curve_type = (&curve.curve_type).into();

  collect_trade_fees(&ctx, trade_fees)?;
  mint_tokens(&ctx, token_amount, signer_seeds)?;
  send_sol_to_curve(&ctx, net_amount, curve_key, curve_acc_info.clone())?;

  let price = curve.price;
  let circulating_supply = curve.circulating_supply;
  let is_complete = curve.is_complete();
  let sol_balance = curve.reserve_token_balance.to_string();

  if is_complete {
    fund_creator_account(&ctx, signer_seeds)?;
    collect_protocol_fees(&ctx, &curve_acc_info)?;
    collect_creator_fees(&ctx, curve_acc_info)?;
    
    // mark the curve as closed
    let curve = &mut ctx.accounts.bonding_curve;
    curve.close_curve();
    
    instrospect_next_ix(&ctx)?;
  }
  
  {
    let buyer = ctx.accounts.buyer.key();
    ctx.accounts.buyer_ata.reload()?;
    let buyer_balance = ctx.accounts.buyer_ata.amount;

    emit_cpi!(BuyEvent {
      curve_type, 
      buyer,
      token: *token,
      sol_amount: spendable_amount.to_string(),
      token_amount: token_amount.to_string(),
      is_complete,
      price: price.to_string(),
      circulating_supply: circulating_supply.to_string(),
      sol_balance,
      buyer_balance: buyer_balance.to_string(),
    });
  }

  Ok(())
}
