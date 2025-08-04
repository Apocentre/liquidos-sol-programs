use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use anchor_spl::token_2022::{mint_to, MintTo};
use crate::instructions::mint::Mint;

#[event]
pub struct LiqMinted {
  curve_token: Pubkey,
  buyer: Pubkey,
  curve_creator: Pubkey,
  buyer_amount: String,
  creator_amount: String,
  buyer_balance: String,
  creator_balance: String,
}

pub fn exec<'info>(
  ctx: Context<'_, '_, '_, 'info, Mint<'info>>,
  curve_token: Pubkey,
  amount: u64,
) -> Result<()> {
  let bonding_curve = &mut ctx.accounts.bonding_curve;
  let spendable_amount = u64::min(bonding_curve.max_accepted_amount()?, amount);

  if spendable_amount > 0 {
    let mint_amount = bonding_curve.process_purchase_return(amount)?;
    let creator_amount = bonding_curve.calc_creator_fee(mint_amount)?;
    let buyer_amount = mint_amount.safe_sub(creator_amount)?;

    let state_key = &ctx.accounts.state.key();
    let seeds: &[&[u8]] = &[
      b"liq_bonding_curve",
      state_key.as_ref(),
      &[bonding_curve.bump],
    ];
    let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

    mint_tokens(
      &ctx,
      ctx.accounts.curve_creator_liq_ata.to_account_info(),
      creator_amount,
      signer_seeds,
    )?;
    mint_tokens(
      &ctx,
      ctx.accounts.buyer_liq_ata.to_account_info(),
      buyer_amount,
      signer_seeds,
    )?;

    let buyer = ctx.accounts.buyer.key();
    ctx.accounts.buyer_liq_ata.reload()?;
    let buyer_balance = ctx.accounts.buyer_liq_ata.amount;
    let curve_creator = ctx.accounts.curve_creator.key();
    ctx.accounts.curve_creator_liq_ata.reload()?;
    let creator_balance = ctx.accounts.curve_creator_liq_ata.amount;

    emit_cpi!(LiqMinted {
      curve_token,
      buyer,
      curve_creator,
      buyer_amount: buyer_amount.to_string(),
      creator_amount: creator_amount.to_string(),
      buyer_balance: buyer_balance.to_string(),
      creator_balance: creator_balance.to_string(),
    });
  }

  Ok(())
}

fn mint_tokens<'info>(
  ctx: &Context<'_, '_, '_, 'info, Mint<'info>>,
  to: AccountInfo<'info>,
  amount: u64,
  signer_seeds: &[&[&[u8]]]
) -> Result<()> {
  let cpi_accounts = MintTo {
    mint: ctx.accounts.liq_token.to_account_info(),
    to,
    authority: ctx.accounts.bonding_curve.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  mint_to(cpi_ctx, amount)?;

  Ok(())
}
