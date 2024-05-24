use anchor_lang::{
  prelude::*,
  solana_program::{
    instruction::Instruction, program::invoke_signed,
  },
};
use anchor_spl::{
  token::{burn, sync_native, Burn, SyncNative},
  token_interface::TokenAccount,
};
use ::borsh::BorshSerialize;
use crate::{
  instructions::move_liquidity::MoveLiquidity, raydium,
};

// create a raydium pool with the current liquidity
fn move_liquidity(
  ctx: &Context<MoveLiquidity>,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;
  let token_key = &ctx.accounts.token;
  let wsol_token_key = &ctx.accounts.wsol_token;
  let token_liquidity = curve.calc_token_amount_to_mint()?;
  let reserve_token_liquidity = curve.net_reserve_token_liquidity()?;

  // Raydium expect token_0 to be smaller that token_1
  let (
    token_0,
    token_1,
    token_0_program,
    token_1_program,
    creator_token_0,
    creator_token_1,
    init_amount_0,
    init_amount_1,
  ) = if token_key.key() < wsol_token_key.key() {
    (
      token_key.to_account_info(),
      wsol_token_key.to_account_info(),
      ctx.accounts.token_2022.to_account_info(),
      ctx.accounts.token_program.to_account_info(),
      ctx.accounts.buyer_ata.to_account_info(),
      ctx.accounts.buyer_wsol_ata.to_account_info(),
      token_liquidity,
      reserve_token_liquidity,
    )
  } else {
    (
      wsol_token_key.to_account_info(),
      token_key.to_account_info(),
      ctx.accounts.token_program.to_account_info(),
      ctx.accounts.token_2022.to_account_info(),
      ctx.accounts.buyer_wsol_ata.to_account_info(),
      ctx.accounts.buyer_ata.to_account_info(),
      reserve_token_liquidity,
      token_liquidity,
    )
  };

  let accounts = vec![
    AccountMeta::new(ctx.accounts.buyer.key(), true),
    AccountMeta::new_readonly(ctx.accounts.amm_config.key(), false),
    AccountMeta::new_readonly(ctx.accounts.raydium_authority.key(), false),
    AccountMeta::new(ctx.accounts.pool_state.key(), false),
    AccountMeta::new_readonly(token_0.key(), false),
    AccountMeta::new_readonly(token_1.key(), false),
    AccountMeta::new(ctx.accounts.lp_mint.key(), false),
    AccountMeta::new(creator_token_0.key(), false),
    AccountMeta::new(creator_token_1.key(), false),
    AccountMeta::new(ctx.accounts.creator_lp_token.key(), false),
    AccountMeta::new(ctx.accounts.token_0_vault.key(), false),
    AccountMeta::new(ctx.accounts.token_1_vault.key(), false),
    AccountMeta::new(ctx.accounts.create_pool_fee.key(), false),
    AccountMeta::new(ctx.accounts.observation_state.key(), false),
    AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
    AccountMeta::new_readonly(token_0_program.key(), false),
    AccountMeta::new_readonly(token_1_program.key(), false),
    AccountMeta::new_readonly(ctx.accounts.associated_token_program.key(), false),
    AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
    AccountMeta::new_readonly(ctx.accounts.rent.key(), false),
  ];

  let mut data: Vec<u8> = Vec::new();

  raydium::InitializeIx {
    init_amount_0,
    init_amount_1,
    open_time: 0,
  }.serialize(&mut data)?;

  let ix = Instruction {
    program_id: raydium::id(),
    accounts,
    data,
  };

  invoke_signed(
    &ix,
    &[
      ctx.accounts.buyer.to_account_info(),
      ctx.accounts.amm_config.to_account_info(),
      ctx.accounts.raydium_authority.to_account_info(),
      ctx.accounts.pool_state.to_account_info(),
      token_0,
      token_1,
      ctx.accounts.lp_mint.to_account_info(),
      creator_token_0,
      creator_token_1,
      ctx.accounts.creator_lp_token.to_account_info(),
      ctx.accounts.token_0_vault.to_account_info(),
      ctx.accounts.token_1_vault.to_account_info(),
      ctx.accounts.create_pool_fee.to_account_info(),
      ctx.accounts.observation_state.to_account_info(),
      ctx.accounts.token_program.to_account_info(),
      token_0_program,
      token_1_program,
      ctx.accounts.associated_token_program.to_account_info(),
      ctx.accounts.system_program.to_account_info(),
      ctx.accounts.rent.to_account_info(),
    ],
    signer_seeds,
  )?;
  
  Ok(())
}

/// sync_native the SOL that was sent in the last Buy transaction. We can't manipulate directly the account
/// through `transfer_from_pda` which directly manipulates accounts and then have a CPI 
/// For move info here https://stackoverflow.com/a/77591006/512783
fn sync_buyer_wsol_ata(ctx: &Context<MoveLiquidity>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let cpi_accounts = SyncNative {
    account: ctx.accounts.buyer_wsol_ata.to_account_info(),
  };
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
  sync_native(cpi_ctx)?;

  Ok(())
}

/// Loads the create_lp_token which is passed as unchecked AccountInfo the program. This is because
/// this is created in the Raydium program so when this program is called the account doesn't exists
/// and thus we can't just use an InterfaceAccount<'info, TokenAccount>.
/// When this function is called we know for sure that the account is created so we just need to load it.
fn get_creator_lp_token(creator_lp_token: &AccountInfo<'_>) -> Result<TokenAccount> {
  let mut data: &[u8] = &creator_lp_token.try_borrow_data()?;
  let account = TokenAccount::try_deserialize(&mut data)?;

  Ok(account)
}

/// Burns the LP created in the move_liquidity. These LP tokens are sent to the buyer
/// whose purchase triggered the liquidity move. We need to burn this liquidity
fn burn_lp(ctx: &Context<MoveLiquidity>) -> Result<()> {
  let creator_lp_token = &ctx.accounts.creator_lp_token;
  let cpi_accounts = Burn {
    mint: ctx.accounts.lp_mint.to_account_info(),
    from: creator_lp_token.to_account_info(),
    authority: ctx.accounts.buyer.to_account_info(),
  };
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
  
  // reload the ata and check the new balance
  let creator_lp_token = get_creator_lp_token(&ctx.accounts.creator_lp_token)?;
  let lp_balance = creator_lp_token.amount;
  
  burn(cpi_ctx, lp_balance)
}


pub fn exec(ctx: Context<MoveLiquidity>) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;
  
  // This Ix might be called even if the pool is completed. Read the docs of `instrospect_next_ix` for more details.
  // We want to act upon only if the curce is completed
  if curve.closed == 1 {
    let token = &ctx.accounts.token.key();
    let state_key = &ctx.accounts.state.key();
    let seeds: &[&[u8]] = &[
      b"bonding_curve",
      state_key.as_ref(),
      token.as_ref(),
      &[curve.bump],
    ];
    let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];

    sync_buyer_wsol_ata(&ctx, signer_seeds)?;
    move_liquidity(&ctx, signer_seeds)?;
    burn_lp(&ctx)?;
  }

  Ok(())
}
