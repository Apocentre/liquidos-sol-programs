
1. Deploy the onlybags and upgrade the onlybags-staking programs:

```bash
solana program deploy --with-compute-unit-price 1500000  --max-sign-attempts 10000 --url https://patient-tiniest-shadow.solana-mainnet.quiknode.pro/9029d4502623bd7390578c2c811a62516ef4a826 --keypair ./wallets/deployer.json --upgrade-authority ./wallets/deployer.json --program-id ./wallets/staking.json target/deploy/onlybags_staking.so
```


```bash
solana program deploy --with-compute-unit-price 2500000  --max-sign-attempts 10000 --url https://patient-tiniest-shadow.solana-mainnet.quiknode.pro/9029d4502623bd7390578c2c811a62516ef4a826 --keypair ./wallets/deployer.json --upgrade-authority ./wallets/deployer.json --program-id ./wallets/onlybags.json target/deploy/onlybags.so
```

2. initialize onlybags program

```bash
ENV=mainnet ANCHOR_WALLET=./wallets/deployer.json node ./scripts/onlybags/initialize.js
```

Set `onlyBagsState` in config.json

3. initialize staking program

```bash
ENV=mainnet ANCHOR_WALLET=./wallets/deployer.json node ./scripts/onlybags-staking/initialize.js
```

Set `stakingState` in config.json

4. update onlybags state

```bash
ENV=mainnet ANCHOR_WALLET=./wallets/deployer.json node ./scripts/onlybags/updateState.js
```
