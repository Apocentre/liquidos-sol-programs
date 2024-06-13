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

### Close Program Account

```bash
solana program close BtqVBXSbaMY6A4ZWHS1HSZZkPvPPwae8w5553F3k6ut --recipient 85Wgv3aHVyrZpMzmyCvd47hNC4g3f25SwJnboDksU86X --keypair ./wallets/deployer.json --bypass-warning
```

> NOTE! Once you delete a program account you can't resuse the same account again! i.e. you can't redeploy the program under the same address.


## Recover After Failed Deployment or Upgrade

The deployment (or upgrade) might not be fully finished for various reasons e.g. ctl+c. This result in multiple intemediate accounts being created which are not closed. Thus quite a bit of SOL will be locked in those account. To restore that balance we need to close the account. There is a command in the cli to do so.

`solana program close --keypair ./wallets/deployer.json --buffers --recipient 3amghT6p74VNhStZRudHdMXFNVhTnswWs1V6my74qR7A`

