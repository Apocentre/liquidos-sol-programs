
use anchor_lang::prelude::*;
use math::decimal_error::DecimalErrorHandler;
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;
use anchor_safe_math::SafeMath;
use crate::constants::SPACE_MARGIN;

#[account]
#[derive(InitSpace, Debug)]
pub struct BondingCurve {
  /// Current circulating supply of the token in the lowest denomination i.e. decimals included
  pub circulating_supply: u64,
  /// The balance of reserve token i.e. SOL in the lowest denomination (lamport) i.e. decimals included
  pub reserve_token_balance: u64,
  /// The LIQ mint account
  pub liq_token: Pubkey,
  /// Current creator fees (BPS). Each time tokens are minted creatorFee will 
  /// be sent to the creator and (BPS - creatorFee) to the buyer
  pub creator_fee_bps: u64,
  /// The PDA bump of this account
  pub bump: u8,
}

pub const ONE_TOKEN: Decimal = dec!(1_000_000);
pub const LAMPORT_IN_SOL: Decimal = dec!(1_000_000_000);

impl BondingCurve {
  pub const MAX_SIZE: usize = 8 + Self::INIT_SPACE + SPACE_MARGIN;
  pub const MAX_SUPPLY: u64 = 50_000_000_000_000; // 50M
  const TARGET: u64 = 833_333_000_000_000; // 833K

  pub fn new(liq_token: Pubkey, creator_fee_bps: u64, bump: u8) -> Self {
    Self {
      circulating_supply: 0,
      reserve_token_balance: 0,
      liq_token,
      creator_fee_bps,
      bump,
    }
  }

  /// We need to account for rounding issues. The new minted amount plus the current supply should not exceed
  /// the total 
  fn calc_mint_account(&self, amount: u64) -> Result<u64> {
    let available = Self::MAX_SUPPLY.safe_sub(self.circulating_supply)?;
    Ok(u64::min(available, amount))
  }  

  pub fn max_accepted_amount(&self) -> Result<u64> {
    Ok(Self::TARGET.safe_sub(self.reserve_token_balance)?)
  }

  pub fn calc_creator_fee(&self, amount: u64) -> Result<u64> {
    let fees = amount
    .safe_mul(self.creator_fee_bps)?
    .safe_div(10_000)?;

    Ok(fees)
  }

  fn update_state(&mut self, tokens_minted: u64, sol_amount: u64) -> Result<()> {
    self.circulating_supply = self.circulating_supply.safe_add(tokens_minted)?;
    self.reserve_token_balance = self.reserve_token_balance.safe_add(sol_amount)?;

    Ok(())
  }

  pub fn process_purchase_return(&mut self, reserve_tokens_received: u64) -> Result<u64> {
    let reserve_tokens_received_dec = Decimal::safe_from_u64(reserve_tokens_received)?;
    let circulating_supply = Decimal::safe_from_u64(self.circulating_supply)?;
    let k = dec!(3);
    let k_exp = k.exp();
    let max_supply = Decimal::safe_from_u64(Self::MAX_SUPPLY)?;
    let target = Decimal::safe_from_u64(Self::TARGET)?;
    let c = target.safe_mul(k)?
      .safe_div(max_supply.safe_mul(k_exp.safe_sub(dec!(1))?)?)?;
    let term = reserve_tokens_received_dec.safe_mul(k)?
      .safe_div(c.safe_mul(max_supply)?)?;
    let exp_term = k.safe_mul(circulating_supply)?.safe_div(max_supply)?.exp();
    let s2 = max_supply.safe_div(k)?
      .safe_mul(exp_term.safe_add(term)?.ln())?;
    let tokens_received = s2.safe_sub(circulating_supply)?;

    let tokens_received = self.calc_mint_account(tokens_received.safe_to_u64()?)?;
    self.update_state(tokens_received, reserve_tokens_received)?;

    Ok(tokens_received)
  }
}
