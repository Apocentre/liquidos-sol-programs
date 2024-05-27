use anchor_lang::prelude::*;

/// We can do this because the from account is a pda which is owned by this program. Otherwise it will fail with
/// `failed to verify account ...: instruction spent from the balance of an account it does not own`
/// Note that the same error appears to be raised when we try to transfer like this
///
/// ```no_run
/// let cpi_context = CpiContext::new_with_signer(
///    ctx.accounts.system_program.to_account_info(),
///    NativeTransfer {from: escrow.clone(), to: treasury.clone()},
///    signer_seeds,
/// );
///  transfer(cpi_context, protocol_fee_amount)?;
/// ```
/// Neither `invoke_signed` works.
pub fn transfer_from_pda(
  from_pda: &mut AccountInfo,
  to: &mut AccountInfo,
  amount: u64,
) -> Result<()> {
  from_pda.sub_lamports(amount)?;
  to.add_lamports(amount)?;

  Ok(())
}

pub fn deser<T: AccountDeserialize>(account: AccountInfo) -> Result<T> {
  let account_data = account.try_borrow_data()?;
  let mut data_slice: &[u8] = &account_data;
  let data = T::try_deserialize(&mut data_slice)?;
  
  Ok(data)
}
