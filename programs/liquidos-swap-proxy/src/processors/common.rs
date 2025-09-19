use anchor_lang::prelude::*;
use anchor_spl::{
  {token::{self, Transfer}, token_2022::{self, TransferChecked}},
  associated_token::{self, Create},
};
use math::utils::calc_perc_value;
use crate::{instructions::swap::Swap, raydium::is_wsol};

pub const TOKEN_DECIMALS: u8 = 6;

pub fn create_treasury_atas<'info>(ctx: &Context<'_, '_, '_, 'info, Swap<'info>>) -> Result<()> {
  let input_token = &ctx.accounts.input_token_mint;
  let output_token = &ctx.accounts.output_token_mint;
  
  // comes in sets of 3 [treasury1, treasury1_input_ata, treasure_output_ata]
  for treasury_accs in ctx.remaining_accounts.chunks(3) {
    // We need to create this here instead of using Anchor macros bacause we don't know
    // that token program each tokens belongs to e.g. token_program or token_2022
    create_ata_if_needed(
      ctx.accounts.payer.to_account_info(),
      treasury_accs[1].clone(),
      treasury_accs[0].clone(),
      input_token.to_account_info(),
      ctx.accounts.system_program.to_account_info(),
      if is_wsol(&input_token.key())? {ctx.accounts.token_program.to_account_info()} else {ctx.accounts.token_2022.to_account_info()},
      ctx.accounts.associated_token_program.to_account_info(),
    )?;
    create_ata_if_needed(
      ctx.accounts.payer.to_account_info(),
      treasury_accs[2].clone(),
      treasury_accs[0].clone(),
      output_token.to_account_info(),
      ctx.accounts.system_program.to_account_info(),
      if is_wsol(&output_token.key())? {ctx.accounts.token_program.to_account_info()} else {ctx.accounts.token_2022.to_account_info()},
      ctx.accounts.associated_token_program.to_account_info(),
    )?;
  }

  Ok(())
}

pub fn create_ata_if_needed<'info>(
  payer: AccountInfo<'info>,
  associated_token: AccountInfo<'info>,
  authority: AccountInfo<'info>,
  mint: AccountInfo<'info>,
  system_program: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
  associated_token_program: AccountInfo<'info>,
) -> Result<()> {
  // check if account already exists
  if associated_token.lamports() == 0 {
    let cpi_accounts = Create {
      payer,
      associated_token,
      authority,
      mint,
      system_program,
      token_program,
    };

    let cpi_ctx = CpiContext::new(associated_token_program, cpi_accounts);

    associated_token::create(cpi_ctx)?;
  }
  
  Ok(())
}

pub fn collect_fees<'info>(
  ctx: &Context<'_, '_, '_, 'info, Swap<'info>>,
  token_amount_received: u64,
) -> Result<()> {
  let state = &ctx.accounts.state;
  let output_token_mint = &ctx.accounts.output_token_mint;
  let fees = calc_perc_value(token_amount_received, state.protocol_fee_bps)?;

  for treasury_accs in ctx.remaining_accounts.chunks(3) {
    let treasury_fee = state.calc_treasury_fee(&treasury_accs[0].key(), fees)?;

    if is_wsol(&output_token_mint.key())? {
      let cpi_accounts = Transfer {
        from: ctx.accounts.output_token_account.to_account_info(),
        to: treasury_accs[2].clone(),
        authority: ctx.accounts.payer.to_account_info(),
      };
    
      let cpi_program = ctx.accounts.token_program.to_account_info();
      let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
      token::transfer(cpi_ctx, treasury_fee)?;
    } else {
      let cpi_accounts = TransferChecked {
        from: ctx.accounts.output_token_account.to_account_info(),
        mint: ctx.accounts.output_token_mint.to_account_info(),
        to: treasury_accs[2].clone(),
        authority: ctx.accounts.payer.to_account_info(),
      };
    
      let cpi_program = ctx.accounts.token_2022.to_account_info();
      let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
      token_2022::transfer_checked(cpi_ctx, treasury_fee, TOKEN_DECIMALS)?;
    }
  }
 
  Ok(())
}
