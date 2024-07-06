
use std::mem::size_of;
use anchor_lang::prelude::*;
use anchor_safe_math::SafeMath;
use crate::curve_formulas::CurveType;

pub const MAX_OPERATORS: usize = 5;

#[account]
#[derive(Debug)]
pub struct BondingCurve {
  /// The type of the curve. This implement the main logic of the curve i.e. the formulas
  pub curve_type: CurveType,
  /// The creator of the token this bonding curve is associated with
  pub token_creator: Pubkey,
  /// The mint account of the token associated with this curve
  pub token: Pubkey,
  /// Current protocol fees (fixed lamports amount).. This is applied when the pool is created on Raydium
  pub protocol_fee: u64,
  /// Current trade fees (BPS). This is applied on each trade that takes place. Fees collected in SOL
  pub trade_fee_bps: u64,
  /// Current creator fees (fixed lamports amount). This is applied when the pool is created on Raydium
  pub creator_fee: u64,
  /// Current circulating supply of the token in the lowest denomination i.e. decimals included
  pub circulating_supply: u64,
  /// The total supply of the newly created tokens in the lowest denomination i.e. decimals included
  pub total_supply: u64,
  /// The balance of reserve token i.e. SOL in the lowest denomination (lamport) i.e. decimals included
  pub reserve_token_balance: u64,
  /// The current price of the curve in lamports
  pub price: u64,
  /// The PDA bump of this account
  pub bump: u8,
  /// Is this pool closed? Closed means sol target reached
  pub closed: u8,
}

impl BondingCurve {
  pub const MAX_SIZE: usize = 8
  + size_of::<Self>();

  pub fn try_new(
    curve_type: u8,
    token_creator: Pubkey,
    token: Pubkey,
    protocol_fee: u64,
    trade_fee_bps: u64,
    creator_fee: u64,
    total_supply: u64,
    bump: u8,
  ) -> Result<Self> {
    Ok(Self {
      curve_type: curve_type.try_into()?,
      token_creator,
      token,
      protocol_fee,
      trade_fee_bps,
      creator_fee,
      circulating_supply: 0,
      total_supply,
      reserve_token_balance: 0,
      price: 0,
      bump,
      closed: 0,
    })
  }

  /// Finds the current price of the curve
  fn calc_price(&self) -> Result<u64> {
    self.curve_type.calc_price(self.circulating_supply)
  }

  /// Calculates the number of tokens to mint based on the given amount of reserve tokens.
  /// This function is used when user buys the token with SOL
  pub fn process_purchase_return(&mut self, reserve_tokens_received: u64) -> Result<u64> {
    let k = self.curve_type.process_purchase_return(reserve_tokens_received, self.circulating_supply)?;

    // update state
    self.circulating_supply = self.circulating_supply.safe_add(k)?;
    self.reserve_token_balance = self.reserve_token_balance.safe_add(reserve_tokens_received)?;
    self.price = self.calc_price()?;

    Ok(k)
  }

  /// Given an amount of tokens, calucates the amount of reserve tokens to be sent back.
  /// This function is used when user sells the tokens and receives back SOL
  pub fn process_sale_return(&mut self, token_amount: u64) -> Result<u64> {
    let reserve_tokens_returned = self.curve_type.process_sale_return(token_amount, self.circulating_supply)?;

    // update state
    self.circulating_supply = self.circulating_supply.safe_sub(token_amount)?;
    self.reserve_token_balance = self.reserve_token_balance.safe_sub(reserve_tokens_returned)?;
    self.price = self.calc_price()?;

    Ok(reserve_tokens_returned) 
  }
  
  /// Calculates the total fees to be paid when funds are migrated to Raydium.
  fn calc_migration_fees(&self) -> Result<u64> {
    let total_fee = self.protocol_fee.safe_add(self.creator_fee)?;
    Ok(total_fee)
  }

  /// Returns the max amount one can send to the curve. It depends on the sol target
  /// and the current amount of tokens in the pool
  pub fn max_accepted_amount(&self) -> Result<u64> {
    let amount = self.curve_type.sol_target().safe_sub(self.reserve_token_balance)?;
    Ok(amount)
  }

  pub fn is_complete(&self) -> bool {
    self.reserve_token_balance == self.curve_type.sol_target()
  }

  pub fn close_curve(&mut self) {
    self.closed = 1;
  }

  pub fn calc_trade_fees(&self, sol_amount: u64) -> Result<u64> {
    let fees = sol_amount
    .safe_mul(self.trade_fee_bps)?
    .safe_div(10_000)?;

    Ok(fees)
  }

  /// Find the net amount of reserve token that can be used as liquidity in the Raydium pool
  pub fn net_reserve_token_liquidity(&self) -> Result<u64> {
    let net = self.reserve_token_balance.safe_sub(self.calc_migration_fees()?)?;

    Ok(net)
  }

  /// We need enough tokens to fill the total supply set for this curve
  pub fn calc_token_amount_to_mint(&self) -> Result<u64> {
    let amount = self.total_supply.safe_sub(self.circulating_supply)?;
    Ok(amount)
  }
}

#[cfg(test)]
mod tests {
  use anchor_lang::solana_program::pubkey::Pubkey;
  use anchor_spl::token_2022::spl_token_2022::solana_zk_token_sdk::curve25519::scalar::Zeroable;

  use super::BondingCurve;

  #[test]
  fn returns_correct_purchase_amount() {
    let mut curve = BondingCurve::try_new(
      1,
      Pubkey::zeroed(),
      Pubkey::zeroed(),
      100,
      1000,
      100,
      100,
      1_000_000_000 * 10e6 as u64,
      1,
    ).unwrap();
    let received = curve.process_purchase_return(89800000000).unwrap();
    assert_eq!(received, 793004689489822);
    assert_eq!(curve.circulating_supply, 793004689489822);
    assert_eq!(curve.reserve_token_balance, 89800000000);
  }

  #[test]
  fn process_sale_return_amount() {
    let mut curve = BondingCurve::try_new(
      1,
      Pubkey::zeroed(),
      Pubkey::zeroed(),
      100,
      1000,
      100,
      100,
      1_000_000_000 * 10e6 as u64,
      1,
    ).unwrap();

    let tokens_received = curve.process_purchase_return(89800000000).unwrap();
    let received = curve.process_sale_return(tokens_received).unwrap();
    assert_eq!(received, 89800000000);
    assert_eq!(curve.circulating_supply, 0);
    assert_eq!(curve.reserve_token_balance, 0);
  }

  #[test]
  fn simulate() {
    let mut curve = BondingCurve::try_new(
      1,
      Pubkey::zeroed(),
      Pubkey::zeroed(),
      100,
      500,
      100,
      100,
      1_000_000_000 * 10e6 as u64,
      1,
    ).unwrap();
    
    for _ in 0..90 {
      let received = curve.process_purchase_return(1_000_000_000).unwrap();
      println!("Sent 1 SOL and Received {:?} Tokens. {:?}", received, curve);
    }
  }

  #[test]
  fn simulate_buy_and_sell() {
    let mut curve = BondingCurve::try_new(
      1,
      Pubkey::zeroed(),
      Pubkey::zeroed(),
      100,
      500,
      100,
      100,
      1_000_000_000 * 10e6 as u64,
      1,
    ).unwrap();
    let received = curve.process_purchase_return(500000000).unwrap(); // 0.5
    println!("Buyer 1 Sent 0.5 SOL and Received {:?} Tokens. {:?}", received, curve);
    let received_2 = curve.process_purchase_return(300000000).unwrap(); // 0.3
    println!("Buyer 2 Sent 0.3 SOL and Received {:?} Tokens. {:?}", received_2, curve);

    // first buyer sells half of the tokens
    let sol_received = curve.process_sale_return(received / 2).unwrap();
    println!("Buyer 1 Sent {} Tokens and Received {:?} SOL. {:?}", received / 2, sol_received, curve);
    
    // second buyer sells 3/4 of the tokens
    let sol_received_2 = curve.process_sale_return((received_2 * 3) / 4).unwrap();
    println!("Buyer 2 Sent {} Tokens and Received {:?} SOL. {:?}", (received_2 * 3) / 4, sol_received_2, curve);
    
    // first buyer buys using the sol received from previous purchase
    let received_1_2 = curve.process_purchase_return(sol_received).unwrap();
    println!("Buyer 1 Sent {} SOL and Received {:?} Tokens. {:?}", sol_received, received, curve);

    // second buyer sells the remaining of his tokens
    let sol_received_2 = curve.process_sale_return(received_2 / 4).unwrap();
    println!("Buyer 2 Sent {} Tokens and Received {:?} SOL. {:?}", received_2 * 1 / 4, sol_received_2, curve);

    // first buyer sells all his tokens
    let buyer_1_tokens = received - (received / 2) + received_1_2;
    let sol_received = curve.process_sale_return(buyer_1_tokens).unwrap();
    println!("Buyer 1 Sent {} Tokens and Received {:?} SOL. {:?}", buyer_1_tokens, sol_received, curve);
    
    assert_eq!(curve.circulating_supply, 0);
    // some rounding errors due to divisions made above. The point is that the amount left is tiny
    assert_eq!(curve.reserve_token_balance, 2);
  }
}
