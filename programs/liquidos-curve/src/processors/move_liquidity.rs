use anchor_lang::{
  prelude::*,
  solana_program::{
    instruction::Instruction, program::invoke,
  },
};
use anchor_safe_math::SafeMath;
use anchor_spl::{
  associated_token, token::{self, Burn, SyncNative, Transfer, burn, sync_native},
  token_2022::{SetAuthority, set_authority, spl_token_2022::instruction::AuthorityType},
  token_interface::TokenAccount,
};
use math::utils::calc_perc_value;
use crate::{
  instructions::move_liquidity::MoveLiquidity,
  raydium::{self, AmmConfig},
};
use super::common::deser;



// create a raydium pool with the current liquidity
fn move_liquidity(ctx: &Context<MoveLiquidity>) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;
  let amm_config: AmmConfig = deser(ctx.accounts.amm_config.clone())?;
  let token_key = &ctx.accounts.token;
  let wsol_token_key = &ctx.accounts.wsol_token;
  let token_liquidity = curve.calc_token_amount_to_mint()?;
  
  // we don't want buyer pay for the pool creation. It has to be funded using the curving pool SOL.
  // In the previous (buy) ix we transfered the create_pool_fee SOL amount to the buyer account.
  let total_amm_cost = amm_config.create_pool_fee.safe_add(raydium::RENT_COST)?;
  let reserve_token_liquidity = curve.net_reserve_token_liquidity()?.safe_sub(total_amm_cost)?;

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

  // add the ix_discriminator so Raydium's Anchor program can identity the instruction
  let mut data: Vec<u8> = vec![175, 175, 109, 31, 13, 152, 155, 237];
  let mut ix_data: Vec<u8> = Vec::new();

  raydium::InitializeIx {
    init_amount_0,
    init_amount_1,
    open_time: 0,
  }.serialize(&mut ix_data)?;

  data.extend(&ix_data);
  let ix = Instruction {
    program_id: ctx.accounts.cp_swap_program.key(),
    accounts,
    data,
  };

  invoke(
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
    ]
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

/// Loads the ata for the given ata address which is passed as unchecked AccountInfo the program. This is because
/// this is created in the Raydium program so when this program is called the account doesn't exists
/// and thus we can't just use an InterfaceAccount<'info, TokenAccount>.
/// When this function is called we know for sure that the account is created so we just need to load it.
fn load_ata(creator_lp_token: &AccountInfo<'_>) -> Result<TokenAccount> {
  let mut data: &[u8] = &creator_lp_token.try_borrow_data()?;
  let account = TokenAccount::try_deserialize(&mut data)?;

  Ok(account)
}

/// Creates on LP ATA for each treasury
fn create_treasury_lp_ata<'info>(
  ctx: &Context<'_, '_, '_, 'info, MoveLiquidity<'info>>,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  let associated_token_program = &ctx.accounts.associated_token_program;

  for treasury in ctx.remaining_accounts {
    let cpi_accounts = associated_token::Create {
      payer: ctx.accounts.buyer.to_account_info(),
      associated_token: treasury.to_account_info(),
      authority: ctx.accounts.bonding_curve.to_account_info(),
      mint: ctx.accounts.token.to_account_info(),
      system_program: ctx.accounts.system_program.to_account_info(),
      token_program: ctx.accounts.token_2022.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(associated_token_program.to_account_info(), cpi_accounts, signer_seeds);
    associated_token::create(cpi_ctx)?;
  }
  
  Ok(())
}

/// Burns 90% of LP created in the move_liquidity. These LP tokens are sent to the buyer
/// whose purchase triggered the liquidity move. We need to burn this liquidity
fn burn_and_distribute_lp<'info>(
  ctx: &Context<'_, '_, '_, 'info, MoveLiquidity<'info>>,
) -> Result<()> {
  let creator_lp_token = &ctx.accounts.creator_lp_token;
  let cpi_accounts = Burn {
    mint: ctx.accounts.lp_mint.to_account_info(),
    from: creator_lp_token.to_account_info(),
    authority: ctx.accounts.buyer.to_account_info(),
  };
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
  
  // reload the ata and check the new balance
  let creator_lp_token = load_ata(&ctx.accounts.creator_lp_token)?;
  let lp_balance = creator_lp_token.amount;
  let lp_to_keep = calc_perc_value(lp_balance, ctx.accounts.state.lp_tokens_to_keep_bps)?;
  
  burn(cpi_ctx, lp_balance.safe_sub(lp_to_keep)?)?;

  let state = &ctx.accounts.state;
  // the first three element are the treasury accounts the other three are their coresponding atas
  for treasury_lp_ata in ctx.remaining_accounts.iter().skip(3) {
    let ata = load_ata(treasury_lp_ata)?;
    
    let cpi_accounts = Transfer {
      from: ctx.accounts.creator_lp_token.to_account_info(),
      to: treasury_lp_ata.to_account_info(),
      authority: ctx.accounts.buyer.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    let amount = state.calc_treasury_fee(&ata.owner, lp_to_keep)?;

    token::transfer(cpi_ctx, amount)?;
  }

  Ok(())
}

fn revoke_mint_authority(ctx: &Context<MoveLiquidity>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
  let cpi_accounts = SetAuthority {
    current_authority: ctx.accounts.bonding_curve.to_account_info(),
    account_or_mint: ctx.accounts.token.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_2022.to_account_info();
  let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

  set_authority(cpi_ctx, AuthorityType::MintTokens, None)
}

pub fn exec<'info>(
  ctx: Context<'_, '_, '_, 'info, MoveLiquidity<'info>>,
) -> Result<()> {
  let curve = &ctx.accounts.bonding_curve;
  
  // This Ix might be called even if the pool is completed. Read the docs of `instrospect_next_ix` for more details.
  // We want to act upon only if the curce is completed
  if curve.closed == 1 {
    let state_key = &ctx.accounts.state.key();
    let token = &ctx.accounts.token.key();
    let seeds: &[&[u8]] = &[
      b"bonding_curve",
      state_key.as_ref(),
      token.as_ref(),
      &[curve.bump],
    ];
    let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];
    
    sync_buyer_wsol_ata(&ctx, signer_seeds)?;
    move_liquidity(&ctx)?;
    create_treasury_lp_ata(&ctx, signer_seeds)?;
    burn_and_distribute_lp(&ctx)?;
    revoke_mint_authority(&ctx, signer_seeds)?;
  }

  Ok(())
}
