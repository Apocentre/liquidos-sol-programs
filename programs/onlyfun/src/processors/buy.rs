use anchor_lang::prelude::*;
use crate::instructions::buy::Buy;

pub fn exec(
  ctx: Context<Buy>,
  amount: u64,
  min_amount_out: u64,
) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;
   require!(curve.sol)

  Ok(())
}
