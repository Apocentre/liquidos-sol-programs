use anchor_lang::prelude::*;
use anchor_spl::token::spl_token::state::Mint;
use crate::account_data::state::State;

#[derive(Accounts)]
pub struct Buy<'info> {
  /// The state account of each instance of this program
  #[account()]
  pub state: Account<'info, State>,

  #[account(mut)]
  pub token: Box<Account<'info, Mint>>,
  
  #[account(mut)]
  pub buyer: Signer<'info>,
}
