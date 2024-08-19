pub mod account_data;
pub mod instructions;
pub mod processors;
pub mod program_error;

use anchor_lang::prelude::*;

declare_id!("8c3Znxt8mLm3kbmJBYkbKJSsEq7SCxDntNgRJeeGbr8W");

#[program]
pub mod onlybags_staking {
  use super::*;


  /// Initialize
  ///
  /// # Arguments
  ///
  /// * `ctx` - The Anchor context holding the accounts
  /// * `treasury` - The treasury account that receives fees
  /// * `protocol_fee` - Current protocol fees (fixed lamports amount). This is applied when the pool is created on Raydium
  /// * `trade_fee_bps` - Current trade fees (BPS). This is applied on each trade that takes place. Fees collected in SOL
  /// * `creator_fee` - Current creator fees (fixed lamports amount). This is applied when the pool is created on Raydium
  /// * `total_token_supply` - The total supply of the newly created tokens in the lowest denomination i.e. decimals included
  pub fn initialize(
    ctx: Context<Initialize>,
    treasury: Pubkey,
    protocol_fee: u64,
    trade_fee_bps: u64,
    creator_fee: u64,
    total_token_supply: u64,
  ) -> Result<()> {
    processors::initialize::exec(
      ctx,
      treasury,
      protocol_fee,
      trade_fee_bps,
      creator_fee,
      total_token_supply,
    )
  }
}
