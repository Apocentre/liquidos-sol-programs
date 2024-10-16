use anchor_lang::prelude::*;

use crate::{
  instructions::init_user_info::InitUserInfo, program_error::ErrorCode,
};

pub fn exec(ctx: Context<InitUserInfo>) -> Result<()> {
  let user_info = &mut ctx.accounts.user_info;
  require!(!user_info.initialized, ErrorCode::UserInfoInitialized);

  user_info.initialized = true;

  Ok(())
}
