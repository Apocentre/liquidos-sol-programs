Upgrading State
===

Along with the program logic upgrade, we might what to upgrade the existing account data. For example, as part of the v2 upgrade process, we changed the existing `bonding_curve` and `state` accounts of the liquidos curve program.

**V1 Account**

```rust
pub struct BondingCurve {
  pub curve_type: CurveType,
  pub token_creator: Pubkey,
  pub token: Pubkey,
  pub protocol_fee: u64,
  pub trade_fee_bps: u64,
  pub creator_fee: u64,
  pub circulating_supply: u64,
  pub total_supply: u64,
  pub reserve_token_balance: u64,
  pub price: u64,
  pub bump: u8,
  pub closed: u8,
}

pub struct State {
  pub owner: Pubkey,
  pub treasury: Pubkey,
  pub protocol_fee: u64,
  pub trade_fee_bps: u64,
  pub creator_fee: u64,
  pub total_token_supply: u64,
}
```

**V2 Account**

```rust
pub struct BondingCurve {
  pub curve_type: CurveType,
  pub token_creator: Pubkey,
  pub token: Pubkey,
  pub protocol_fee: u64,
  pub trade_fee_bps: u64,
  pub creator_fee: u64,
  pub circulating_supply: u64,
  pub total_supply: u64,
  pub reserve_token_balance: u64,
  pub price: u64,
  pub bump: u8,
  pub closed: u8,
+ pub staking_allocation: u64,
}

pub struct State {
  pub owner: Pubkey,
  pub treasury: Pubkey,
  pub protocol_fee: u64,
  pub trade_fee_bps: u64,
  pub creator_fee: u64,
  pub total_token_supply: u64,
+ pub staking_program: Option<Pubkey>,
+ pub staking_program_state: Option<Pubkey>,
+ pub staking_allocation: u64,
}
```

> We should always add fields to the end of the struct. This will guarantee that the existing accounts can be deserialized properly.

Upgrading the program logic and running the instruction with the new account data will fail. The error we will get when calling the `buy` instruction, for example, is

```bash
"Program log: AnchorError caused by account: bonding_curve. Error Code: AccountDidNotDeserialize.
```

The reason is simple. The new program logic will try to deserialize the `bonding_curve` using the V2 `BongingCurve` account data. However, it's shape is in line with the V1 `BongingCurve` and thus the deserialization fails. More specifically, the reason the deserialization fails is because the two `BongingCurve` struct have different size. 

To make the old accounts work with the new ones, we have to resize them. To do so we need to use the `realloc` instruction. In anchor this is done though anchor macros. We need to create a new instruction that look like this:

```rust
#[derive(Accounts)]
#[instruction(size: u64)]
pub struct ResizeBondingCurve<'info> {
  /// The state account of each instance of this program
  #[account()]
  pub state: Account<'info, Migration<State>>,

  #[account(
    mut,
    realloc = size as usize,
    realloc::payer = payer,
    realloc::zero = false,
  )]
  pub bonding_curve: Account<'info, Migration<BondingCurve>>,

  #[account(
    mut,
    constraint = payer.key() == Pubkey::from_str("DxVMyJ9YGahVLDXwEb5RaWcFx89JcAErCYGTJrPrneiw").unwrap() @ ErrorCode::OnlyOwner,
  )]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
}

  pub fn resize_bonding_curve(_ctx: Context<ResizeBondingCurve>, _size: u64) -> Result<()> {
    Ok(())
  }
```

> Note, if we do not do `_ctx: Context<ResizeBondingCurve>` but instead `_: Context<ResizeBondingCurve>`, anchor will not include this function in the binary nor in the IDL


## What is the Migration Account

If you notice above we wrapped the account we want to resize in the `Migration` account. The reason we did this is the same reason why we intended to resize the accounts in the first place.

Running `ResizeBondingCurve` will fail with the same exact error that we describe above.

```bash
"Program log: AnchorError caused by account: bonding_curve. Error Code: AccountDidNotDeserialize.
```

The reason is that anchor will still try to deserialize the `bonding_curve` using the V2 version which is bigger in size the the v1 and thus it will fail.

The workaround is using `Migration` account with a custom implementation of `AccountDeserialize::try_deserialize_unchecked` which does do anything. It doesn't try to deserialize the account and thus the check passes.

## Run the resize script

What is left to do is to simple run the resize script.

```bash
ENV=mainnet ANCHOR_WALLET=./wallets/deployer.json node ./scripts/liquidos-curve/resizeBondingCurve.js
```

Now we can use the old V1 accounts with the new program logic that uses the V2 accounts.

### Issue with the new IDL

The `Migration` uses a generic type which breaks our borsh serialization on the JS side. The workaround is to replace the IDL type definition with the following:

```json
    {
      "name": "Migration",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "migrated_type",
            "type": "pubkey"
          }
        ]
      }
    }
```
