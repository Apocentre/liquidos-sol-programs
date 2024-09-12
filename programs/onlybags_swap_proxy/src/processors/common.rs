use anchor_lang::prelude::*;
use anchor_spl::associated_token::{self, Create};

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
