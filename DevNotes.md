## Revoce locked funds during failed deployment or upgrade

To view all buffer accounts that are currently open

```bash
solana program show --buffer-authority DxVMyJ9YGahVLDXwEb5RaWcFx89JcAErCYGTJrPrneiw --buffers --url https://intensive-alien-wish.solana-mainnet.quiknode.pro/77ca2cba4139e49efe0c0a891d16b2cfa31735bc/
```

Similarly you can view all to the program accounts for the given authority.

```bash
solana program show --buffer-authority DxVMyJ9YGahVLDXwEb5RaWcFx89JcAErCYGTJrPrneiw --programs --url https://intensive-alien-wish.solana-mainnet.quiknode.pro/77ca2cba4139e49efe0c0a891d16b2cfa31735bc/
```

To close all the buffer accounts associated with the current authority:

```bash
solana program close --buffers --keypair ./wallets/deployer.json --url https://intensive-alien-wish.solana-mainnet.quiknode.pro/77ca2cba4139e49efe0c0a891d16b2cfa31735bc/
```

More on this here 

https://docs.solanalabs.com/cli/examples/deploy-a-program#closing-program-and-buffer-accounts-and-reclaiming-their-lamports

## Error While Upading

If the size of the original program get bigger than the initial size that was allocated during the initial deployment then the upgrade will fail with this error:

`account data too small for instruction`

TO fix that we need to increase the size of the program account.

`solana program extend {program_id} {add_number_of_bytes} -u devnet --keypair ./wallets/deployer.json`

Note `add_number_of_bytes` will be added to the existing account size; It's not the new size.


## Calculate Mint LEN

In the code we set this value 

`const MAX_TOKEN_SIZE: usize = 234;`

as the size of the Mint account we create.

We found it using the js library. More specifically:

```js
  const extensions = [spl.ExtensionType.MetadataPointer];
  const mintLen = spl.getMintLen(extensions);
```

## Recover After Failed Deployment or Upgrade

The deployment (or upgrade) might not be fully finished for various reasons e.g. ctl+c. This result in multiple intemediate accounts being created which are not closed. Thus quite a bit of SOL will be locked in those account. To restore that balance we need to close the account. There is a command in the cli to do so.

`solana program close --keypair ./wallets/deployer.json --buffers --recipient 85Wgv3aHVyrZpMzmyCvd47hNC4g3f25SwJnboDksU86X`


## Extend program account size

solana program extend 7vLXAAhUcPE4YR5HnJtRPf9cumpYuR43fukAh9XjLUD4  1000 -u devnet --keypair ./wallets/deployer.json


## Revocer deployemnt

```
=========================================================================
Recover the intermediate account's ephemeral keypair file with
`solana-keygen recover` and the following 12-word seed phrase:
=========================================================================
blast surprise pluck country ramp milk blue ranch permit wash wave entire
=========================================================================
To resume a deploy, pass the recovered keypair as the
[BUFFER_SIGNER] to `solana program deploy` or `solana program write-buffer'.
Or to recover the account's lamports, pass it as the
[BUFFER_ACCOUNT_ADDRESS] argument to `solana program close`.
=========================================================================
Error: Error processing Instruction 2: account data too small for instruction
```

1. Revover priv key

```bash
solana-keygen recover --outfile ./temp.json
```

2. resume

```bash
solana program deploy --buffer ./temp.json --with-compute-unit-price 2500000  --max-sign-attempts 10000 --url https://little-thrilling-forest.solana-mainnet.quiknode.pro/b0f644f4dec0fb4c9c47cadf2eb99cafa7356ca8 --keypair ./wallets/deployer.json --upgrade-authority ./wallets/deployer.json --program-id ./wallets/staking.json target/deploy/onlybags_staking.so
```

### Close Program Account (DANGEROUS!!!)

```bash
solana program close 7vLXAAhUcPE4YR5HnJtRPf9cumpYuR43fukAh9XjLUD4 --recipient DxVMyJ9YGahVLDXwEb5RaWcFx89JcAErCYGTJrPrneiw --keypair ./wallets/deployer.json --bypass-warning
```

> NOTE! Once you delete a program account you can't resuse the same account again! i.e. you can't redeploy the program under the same address.

