use std::str::FromStr;
use anchor_lang::{prelude::*, solana_program::sysvar};
use anchor_lang::{
  system_program::{allocate, assign, create_account, Allocate, Assign, CreateAccount, Transfer,}
};
use anchor_spl::{
  token::Token, token_interface::{TokenInterface, Mint, TokenAccount},
  associated_token::AssociatedToken,
};
use crate::bpf_writer::BpfWriter;
use crate::ID;
use crate::{
  account_data::{bonding_curve::BondingCurve, buy_state::BuyState, state::State},
  program_error::ErrorCode, raydium,
};

#[derive(Accounts)]
#[event_cpi]
pub struct Buy<'info> {
  #[account(mut)]
  pub buyer: Signer<'info>,

  /// The state account of each instance of this program
  #[account()]
  pub state: Box<Account<'info, State>>,

  /// CHECK: The treasury account that collects the protocol fees
  #[account(
    mut,
    constraint = treasury.key() == state.treasury @ ErrorCode::WrongTreasury,
  )]
  pub treasury: AccountInfo<'info>,

  /// CHECK: The creator of the curve that collects the creator fees
  #[account(
    mut,
    constraint = token_creator.key() == bonding_curve.token_creator @ ErrorCode::WrongTokenCreator,
  )]
  pub token_creator: AccountInfo<'info>,

  /// The state of the bonding curve that will be used during buys and sells
  #[account(
    mut,
    seeds = [b"bonding_curve", state.key().as_ref(), token.key().as_ref()],
    bump = bonding_curve.bump,
  )]
  pub bonding_curve: Box<Account<'info, BondingCurve>>,

  /// CHECK: the buy_state. All checks take place in the processor
  #[account(
    mut,
    seeds = [b"buy_state", token.key().as_ref(), buyer.key().as_ref()],
    bump,
  )]
  pub buy_state: AccountInfo<'info>,

  /// The ATA of the WSOL token that is owned by the buyer. Create one if no already exists
  #[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = wsol_token,
    associated_token::authority = buyer,
  )]
  pub buyer_wsol_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  /// CHECK: the wsol token account
  #[account(
    constraint = wsol_token.key() == Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
  )]
  pub wsol_token: AccountInfo<'info>,
  
  #[account(
    mut,
    constraint = token.key() == bonding_curve.token @ ErrorCode::InvalidCurveToken,
  )]
  pub token: Box<InterfaceAccount<'info, Mint>>,

  /// CHECK: Which config the pool that will created belongs to. Checks will take place in CP swap program
  #[account(
    address = raydium::amm_config(),
  )]
  pub amm_config: AccountInfo<'info>,

  /// The ATA of the  token that is owned by the buyer. Create one if no already exists
  #[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = token,
    associated_token::authority = buyer,
    associated_token::token_program = token_2022,
  )]
  pub buyer_ata: Box<InterfaceAccount<'info, TokenAccount>>,

  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_2022: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,
  /// CHECK: custom constrain to check the correctness of the account address
  #[account(address = sysvar::instructions::id())]
  pub ix_sysvar: UncheckedAccount<'info>,
}


impl<'info> Buy<'info> {
  pub fn create_buy_state_if_needed(&self, buy_amount: u64, bump: u8) -> Result<()> {
    let actual_owner = self.buy_state.owner;
    let current_lamports = self.buy_state.lamports();
    let rent = Rent::get()?;
    let space = BuyState::MAX_SIZE;

    let mut buy_state_acc: BuyState = if actual_owner == &anchor_lang::solana_program::system_program::ID {
      // create the account
      if current_lamports == 0 {
        let cpi_accounts = CreateAccount {
          from: self.buyer.to_account_info(),
          to: self.buy_state.clone(),
        };
        let cpi_context = CpiContext::new(
          self.system_program.to_account_info(),
          cpi_accounts,
        );

        let lamports = rent.minimum_balance(space);
        create_account(
          cpi_context.with_signer(
            &[
              &[
                b"buy_state",
                self.token.key().as_ref(),
                self.buyer.key().as_ref(),
                &[bump][..],
              ][..],
            ],
          ),
          lamports,
          space as u64,
          &ID,
        )?;
      } else {
        let required_lamports = rent.minimum_balance(space)
          .max(1)
          .saturating_sub(current_lamports);

        if required_lamports > 0 {
          let cpi_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.buy_state.to_account_info(),
          };
          let cpi_context = anchor_lang::context::CpiContext::new(
            self.system_program.to_account_info(),
            cpi_accounts,
          );
          anchor_lang::system_program::transfer(cpi_context, required_lamports)?;
        }

        let cpi_accounts = Allocate {
          account_to_allocate: self.buy_state.to_account_info(),
        };
        let cpi_context = anchor_lang::context::CpiContext::new(
            self.system_program.to_account_info(),
            cpi_accounts,
        );
        allocate(
          cpi_context.with_signer(
            &[
              &[
                b"buy_state",
                self.token.key().as_ref(),
                self.buyer.key().as_ref(),
                &[bump][..],
              ][..],
            ],
          ),
          space as u64,
        )?;
        let cpi_accounts = Assign {
          account_to_assign: self.buy_state.to_account_info(),
        };
        let cpi_context = anchor_lang::context::CpiContext::new(
          self.system_program.to_account_info(),
          cpi_accounts,
        );
        assign(
          cpi_context.with_signer(
            &[
              &[
                b"buy_state",
                self.token.key().as_ref(),
                self.buyer.key().as_ref(),
                &[bump][..],
              ][..],
            ],
          ),
          &ID,
        )?;
      }

      let mut data: &[u8] = &self.buy_state.try_borrow_data()?;
      BuyState::try_deserialize_unchecked(&mut data)?
    } else {
      let mut data: &[u8] = &self.buy_state.try_borrow_data()?;
      BuyState::try_deserialize(&mut data)?
    };

    if actual_owner != &ID {
      return Err(
        Error::from(anchor_lang::error::ErrorCode::ConstraintOwner)
        .with_account_name("buy_state")
        .with_pubkeys((*actual_owner, ID)),
      );
    }

    let required_lamports = rent.minimum_balance(space);
    if self.buy_state.lamports() < required_lamports {
      return Err(
        Error::from(anchor_lang::error::ErrorCode::ConstraintRentExempt)
        .with_account_name("buy_state"),
      );
    }

    buy_state_acc.buy_amount = buy_amount;

    // persist the data changes
    let mut data = self.buy_state.try_borrow_mut_data()?;
    let dst: &mut [u8] = &mut data;
    let mut writer = BpfWriter::new(dst);
    buy_state_acc.try_serialize(&mut writer)?;

    Ok(())
  }
}
