use anchor_lang::prelude::*;
use crate::{
  account_data::bonding_curve::BondingCurve,
  instructions::create_token::CreateToken,
};

pub fn exec(
  ctx: Context<CreateToken>,
) -> Result<()> {
  let token_creator = ctx.accounts.token_creator.key();
  let curve = &mut ctx.accounts.bonding_curve;
  **curve = BondingCurve::new(token_creator, ctx.bumps.bonding_curve);

  Ok(())
}
