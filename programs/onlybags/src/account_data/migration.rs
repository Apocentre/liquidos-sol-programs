use anchor_lang::{prelude::*, Discriminator};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Migration<T>
where
  T: Discriminator,
{
  migrated_type: Option<T>,
}

impl<T> Discriminator for Migration<T>
where
  T: Discriminator,
{
  const DISCRIMINATOR: &'static [u8] = T::DISCRIMINATOR;
}

impl<T> AccountSerialize for Migration<T> where T: Discriminator {}

impl<T> AccountDeserialize for Migration<T>
where
  T: Discriminator,
{
  fn try_deserialize_unchecked(_: &mut &[u8]) -> Result<Self> {
    Ok(Migration::<T> {
      migrated_type: None,
    })
  }

  fn try_deserialize(buf: &mut &[u8]) -> Result<Self> {
    if buf.len() < 8 {
      return Err(ErrorCode::AccountDiscriminatorNotFound.into());
    }
    
    let given_disc = &buf[..8];
    if <T as Discriminator>::DISCRIMINATOR != given_disc {
      return Err(error!(ErrorCode::AccountDiscriminatorMismatch).with_account_name("migration account"));
    }

    Self::try_deserialize_unchecked(buf)
  }
}

impl<T> Owner for Migration<T>
where
  T: Discriminator,
{
  fn owner() -> Pubkey {
    crate::ID
  }
}
