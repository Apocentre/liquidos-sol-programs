
use std::mem::size_of;
use anchor_lang::prelude::*;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use anchor_safe_math::SafeMath;
use crate::math::decimal_error::DecimalErrorHandler;

pub const MAX_OPERATORS: usize = 5;

#[account]
#[derive(Debug)]
pub struct BondingCurve {
  /// The creator of the token this bonding curve is associated with
  pub token_creator: Pubkey,
  /// Target of SOL each pool should receive
  pub sol_target: u64,
  /// Total supply of the token in the lowest denomination i.e. decimals included
  pub total_supply: u64,
  /// The balance of reserve token i.e. SOL in the lowest denomination (lamport) i.e. decimals included
  pub reserve_token_balance: u64,
  /// The current price of the curve in lamports
  pub price: u64,
  /// The PDA bump of this account
  pub bump: u8,
}

impl BondingCurve {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();

  const ONE_TOKEN: Decimal = dec!(1_000_000);
  const LAMPORT_IN_SOL: Decimal = dec!(1_000_000_000);

  pub fn new(
    token_creator: Pubkey,
    sol_target: u64,
    bump: u8,
  ) -> Self {
    Self {
      token_creator,
      sol_target,
      total_supply: 0,
      reserve_token_balance: 0,
      price: 0,
      bump,
    }
  }

  /// Finds the current price of the curve
  fn calc_price(&self) -> Result<u64> {
    let a = dec!(3.34315523).safe_mul(dec!(10).safe_powd(dec!(-9))?)?;
    let b = dec!(17.5970429);
    let total_supply = Self::normalize_token_amount(self.total_supply)?;

    let p = std::f64::consts::E.powf(a.safe_mul(total_supply)?.safe_sub(b)?.safe_to_f64()?);
    let p = Decimal::safe_from_f64(p)?
    .safe_mul(Self::LAMPORT_IN_SOL)?
    .safe_to_u64()?;

    Ok(p)
  }

  /// Calculates the number of tokens to mint based on the given amount of reserve tokens.
  /// This function is used when user buys the token with SOL
  pub fn calculate_purchase_return(&mut self, reserve_tokens_received: u64) -> Result<u64> {
    // divide by 10e9 to convert lamports to SOL
    let reserve_tokens_received_sol = Self::normalize_sol_amount(reserve_tokens_received)?;

    let a = dec!(3.34315523).safe_mul(dec!(10).safe_powd(dec!(-9))?)?;
    let b = dec!(17.5970429);
    let c = dec!(299215564.8);
    // divide by 10e6 to convert token amount to the highest denomination
    let total_supply = Self::normalize_token_amount(self.total_supply)?;
    let d = a.safe_mul(total_supply)?.safe_sub(b)?.safe_to_f64()?;
    let e = std::f64::consts::E.powf(d);
    let e = Decimal::safe_from_f64(e)?;
    
    let k = reserve_tokens_received_sol.safe_div(c)?
    .safe_add(e)?
    .safe_ln()?
    .safe_add(b)?
    .safe_div(a)?
    .safe_sub(total_supply)?
    .safe_mul(Self::ONE_TOKEN)?
    .safe_to_u64()?;

    // update state
    self.total_supply = self.total_supply.safe_add(k)?;
    self.reserve_token_balance = self.reserve_token_balance.safe_add(reserve_tokens_received)?;
    self.price = self.calc_price()?;

    Ok(k)
  }

  /// Given an amount of tokens, calucates the amount of reserve tokens to be sent back.
  /// This function is used when user sells the tokens and receives back SOL
  pub fn calculate_sale_return(&mut self, tokens_sold: u64) -> Result<u64> {
    let a = dec!(3.34315523).safe_mul(dec!(10).safe_powd(dec!(-9))?)?;
    let b = dec!(17.5970429);
    let c = dec!(299215564.8);
    let total_supply = Self::normalize_token_amount(self.total_supply)?;
    let tokens_sold_normalized = Self::normalize_token_amount(tokens_sold)?;

    let d = Decimal::safe_from_f64(std::f64::consts::E.powf(
      a.safe_mul(total_supply.safe_sub(tokens_sold_normalized)?)?.safe_sub(b)?.safe_to_f64()?
    ))?;

    let e = Decimal::safe_from_f64(std::f64::consts::E.powf(
      a.safe_mul(total_supply)?.safe_sub(b)?.safe_to_f64()?
    ))?;

    let reserve_tokens_returned = c.safe_mul(d.safe_sub(e)?)?
    .safe_mul(dec!(-1))?
    .safe_mul(Self::LAMPORT_IN_SOL)?
    .safe_to_u64()?;

    // update state
    self.total_supply = self.total_supply.safe_sub(tokens_sold)?;
    self.reserve_token_balance = self.reserve_token_balance.safe_sub(reserve_tokens_returned)?;
    self.price = self.calc_price()?;

    Ok(reserve_tokens_returned) 
  }

  fn normalize_token_amount(amount: u64) -> Result<Decimal> {
    let value = Decimal::safe_from_u64(amount)?.safe_div(Self::ONE_TOKEN)?;
    Ok(value)
  }

  fn normalize_sol_amount(amount: u64) -> Result<Decimal> {
    let value = Decimal::safe_from_u64(amount)?.safe_div(Self::LAMPORT_IN_SOL)?;
    Ok(value)
  }

  /// Returns the max amount one can send to the curve. It depends on the sol target
  /// and the current amount of tokens in the pool
  pub fn max_accepted_amount(&self) -> Result<u64> {
    let amount = self.sol_target.safe_sub(self.reserve_token_balance)?;
    Ok(amount)
  }

  pub fn is_complete(&self) -> bool {
    self.reserve_token_balance == self.sol_target
  }

  /// We need to mint enough tokens so that the current price is preserved when liquidity
  /// moves to a constant product curve (Raydium).
  /// The equations is y = x / P
  pub fn calculate_token_amount_to_mint(&self) -> Result<u64> {
    let price = Decimal::safe_from_u64(self.price)?;
    let reserve_token_balance = Decimal::safe_from_u64(self.reserve_token_balance)?;
    let amount = reserve_token_balance.safe_div(price)?.safe_mul(Self::ONE_TOKEN)?;

    Ok(amount.safe_to_u64()?)
  }
}

#[cfg(test)]
mod tests {
    use anchor_lang::solana_program::pubkey::Pubkey;
    use anchor_spl::token_2022::spl_token_2022::solana_zk_token_sdk::curve25519::scalar::Zeroable;

    use super::BondingCurve;

  #[test]
  fn returns_correct_purchase_amount() {
    let mut curve = BondingCurve::new(Pubkey::zeroed(), 100, 1, 2);
    let received = curve.calculate_purchase_return(89800000000).unwrap();
    assert_eq!(received, 793004689489822);
    assert_eq!(curve.total_supply, 793004689489822);
    assert_eq!(curve.reserve_token_balance, 89800000000);

    panic!();
  }

  #[test]
  fn calculate_sale_return_amount() {
    let mut curve = BondingCurve::new(Pubkey::zeroed(), 100, 1, 2);

    curve.calculate_purchase_return(89800000000).unwrap();
    let received = curve.calculate_sale_return(793004689489822).unwrap();
    assert_eq!(received, 89800000000);
    assert_eq!(curve.total_supply, 0);
    assert_eq!(curve.reserve_token_balance, 0);
  }

  #[test]
  fn simulate() {
    let mut curve = BondingCurve::new(Pubkey::zeroed(), 100, 1, 2);
    
    for _ in 0..89 {
      let received = curve.calculate_purchase_return(1_000_000_000).unwrap();
      println!("Sent 1 SOL and Received {:?} Tokens. {:?}", received, curve);
    }
    // panic!()
  }
}
