use anchor_lang::prelude::*;

use crate::instructions::init_user_info::InitUserInfo;

pub fn exec(ctx: Context<InitUserInfo>) -> Result<()> {
  let user_info = &mut ctx.accounts.user_info;
  user_info.initialized = true;

  Ok(())
}
