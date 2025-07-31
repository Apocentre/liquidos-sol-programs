#![allow(unexpected_cfgs)]
pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;
pub mod constants;

use anchor_lang::prelude::*;
use instructions::{initialize::*, mint::*};

declare_id!("DP85qMfFbbi1PuotvRmD4mBTekdxDiL1vL68uDSCa6Gz");

#[program]
pub mod liq {
  use super::*;

  /// Initialize
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `name` - The name of the token (used in the metadata account)
  /// * `symbol` - The symbol of the token (used in the metadata account)
  /// * `uri` - The uri of the token (used in the metadata account)
  /// * `liquidos_curve_program` - The address of the main liquidos curve program that will be CPIing into this program
  /// * `liquidos_curve_state` - The state of the main liquidos curve program
  /// * `create_fee_bps` - The current creator fees (BPS)
  pub fn initialize(
    ctx: Context<Initialize>,
    name: String,
    symbol: String,
    uri: String,
    liquidos_curve_program: Pubkey,
    liquidos_curve_state: Pubkey,
    creator_fee_bps: u64,
  ) -> Result<()> {
    processors::initialize::exec(
      ctx,
      liquidos_curve_program,
      liquidos_curve_state,
      creator_fee_bps,
      name,
      symbol,
      uri,
    )
  }

  /// Mint
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `curve_token` - The address of the main liquidos curve program that will be CPIing into this program
  /// * `amount` - The amount of SOL the buyer purchased in the sourve curve
  pub fn mint<'info>(
    ctx: Context<'_, '_, '_, 'info, Mint<'info>>,
    curve_token: Pubkey,
    amount: u64,
  ) -> Result<()> {
    processors::mint::exec(ctx, curve_token, amount)
  }
}
