## Onlybags Programs

## Production Config
The following `config.json` was used in production

```json
{
  "state": "5mjdxZLTXUp61HTXh4bNXtCR5wzgy5XW4HMbkJX6hFRJ",
  "treasury": "A7VthQSmR24rhShWatSiGBnaQVTbshzizLSoRNZPZbLJ",
  "protocolFee": 0,
  "creatorFee": 2000000000,
  "tradeFeeBps": 100,
  "totalTokenSupply": 1000000000000000,
  "tokenCreator": "85Wgv3aHVyrZpMzmyCvd47hNC4g3f25SwJnboDksU86X"
}
```


## Devnet

Config.json

```json
{
  "onlyBagsState": "3SC4Mj5p1EJhZxg6UQp2212WJKdgeGynSCXxTpyV9KsP",
  "treasury": "2Xp8fgVPP8WWVZLSZU9peMN2vZLPgvzxMVzdvHTRZugz",
  "protocolFee": 0,
  "creatorFee": 100000000,
  "tradeFeeBps": 100,
  "stakingAllocationBps": 500,
  "totalTokenSupply": 1000000000000000,
  "tokenCreator": "85Wgv3aHVyrZpMzmyCvd47hNC4g3f25SwJnboDksU86X",
  "stakingToken": "5FxCfNYzW1jdsLpZEuHsMjinFCgqVPaspiS7rtNA3W3m",
  "stakingState": "GbLj2LJWiBqm6ydDKGyixwtC6k42LLE4DNt8Xz2daPs8",
  "stakingDuration": 86400,
  "stakingProtocolFee": 100
}
```


1. deploy onlybags program

```bash
solana program deploy --with-compute-unit-price 1000000  --max-sign-attempts 1000 --url https://smart-convincing-gadget.solana-devnet.quiknode.pro/d014cd6f46918d8dca9642ca6905c92a63096884/ --keypair ./wallets/deployer_devnet.json --upgrade-authority ./wallets/deployer_devnet.json --program-id ./wallets/test/onlybags_v2.json target/deploy/onlybags.so
```

2. deploy staking program

```bash
solana program deploy --with-compute-unit-price 1000000  --max-sign-attempts 1000 --url https://smart-convincing-gadget.solana-devnet.quiknode.pro/d014cd6f46918d8dca9642ca6905c92a63096884/ --keypair ./wallets/deployer_devnet.json --upgrade-authority ./wallets/deployer_devnet.json --program-id ./wallets/test/staking.json target/deploy/onlybags_staking.so
```

3. deploy swap program

```bash
solana program deploy --with-compute-unit-price 1000000  --max-sign-attempts 1000 --url https://smart-convincing-gadget.solana-devnet.quiknode.pro/d014cd6f46918d8dca9642ca6905c92a63096884/ --keypair ./wallets/deployer_devnet.json --upgrade-authority ./wallets/deployer_devnet.json --program-id ./wallets/test/swap_proxy.json target/deploy/onlybags_swap_proxy.so
```


4. deploy locker program

```bash
solana program deploy --with-compute-unit-price 1000000  --max-sign-attempts 1000 --url https://smart-convincing-gadget.solana-devnet.quiknode.pro/d014cd6f46918d8dca9642ca6905c92a63096884/ --keypair ./wallets/deployer_devnet.json --upgrade-authority ./wallets/deployer_devnet.json --program-id ./wallets/test/locker.json target/deploy/onlybags_locker.so
```

5. initialize onlybags program

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/onlybags/initialize.js
```

Set `onlyBagsState` in config.json

6. create test token

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/createToken.js
```

Set `stakingToken` in config.json

7. initialize staking program

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/onlybags-staking/initialize.js
```

Set `stakingState` in config.json

8. update staking state

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/onlybags-staking/updateState.js
```

9. update onlybags state

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/onlybags/updateState.js
```

10. initialize swap program

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/swap-proxy/initialize.js
```

Set `stakingState` in config.json

11. create bonding curve

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/onlybags/createToken.js
```
