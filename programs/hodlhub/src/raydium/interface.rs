use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// Address paying to create the pool. Can be anyone
    #[account(mut)]
    pub creator: Signer<'info>,

    /// Check: Which config the pool belongs to.
    pub amm_config: AccountInfo<'info>,

    /// CHECK: pool vault and lp mint authority
    #[account()]
    pub authority: AccountInfo<'info>,

    /// CHECK: Initialize an account to store the pool state
    #[account(mut)]
    pub pool_state: AccountInfo<'info>,

    /// CHECK: Token_0 mint, the key must smaller then token_1 mint.
    #[account()]
    pub token_0_mint: AccountInfo<'info>,

    /// CHECK: Token_1 mint, the key must grater then token_0 mint.
    #[account()]
    pub token_1_mint: AccountInfo<'info>,

    /// CHECK: pool lp mint
    #[account()]
    pub lp_mint: AccountInfo<'info>,

    /// CHECK: payer token0 account
    #[account(mut)]
    pub creator_token_0: AccountInfo<'info>,

    /// CHECK: creator token1 account
    #[account(mut)]
    pub creator_token_1: AccountInfo<'info>,

    /// CHECK: creator lp token account
    #[account()]
    pub creator_lp_token: AccountInfo<'info>,

    /// CHECK: Token_0 vault for the pool
    #[account(mut)]
    pub token_0_vault: AccountInfo<'info>,

    /// CHECK: Token_1 vault for the pool
    #[account(mut)]
    pub token_1_vault: AccountInfo<'info>,

    /// CHECK: create pool fee account
    #[account(mut)]
    pub create_pool_fee: AccountInfo<'info>,

    /// CHECK: an account to store oracle observations
    #[account()]
    pub observation_state: AccountInfo<'info>,

    /// CHECK: Program to create mint account and mint tokens
    pub token_program: AccountInfo<'info>,
    /// CHECK: Spl token program or token program 2022
    pub token_0_program: AccountInfo<'info>,
    /// CHECK: Spl token program or token program 2022
    pub token_1_program: AccountInfo<'info>,
    /// CHECK: Program to create an ATA for receiving position NFT
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK: To create a new program account
    pub system_program: AccountInfo<'info>,
    /// CHECK: Sysvar for program account
    pub rent: AccountInfo<'info>,
}
