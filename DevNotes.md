# Error While Upading

If the size of the original program get bigger than the initial size that was allocated during the initial deployment then the upgrade will fail with this error:

`account data too small for instruction`

TO fix that we need to increase the size of the program account.

`solana program extend {program_id} {add_number_of_bytes} -u devnet --keypair ./wallets/deployer.json`

Note `add_number_of_bytes` will be added to the existing account size; It's not the new size.
