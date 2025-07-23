> For devnet use the `wallets/deployer_devnet.json` wallet

## v1

1. Deploy the onlybags and onlybags-staking programs:

```bash
solana program deploy --with-compute-unit-price 100000  --max-sign-attempts 10000 --url https://patient-tiniest-shadow.solana-mainnet.quiknode.pro/9029d4502623bd7390578c2c811a62516ef4a826/ --keypair ./wallets/deployer.json --upgrade-authority ./wallets/deployer.json --program-id ./wallets/staking.json target/deploy/onlybags_staking.so
```

```bash
solana program deploy --with-compute-unit-price 100000  --max-sign-attempts 10000 --url https://patient-tiniest-shadow.solana-mainnet.quiknode.pro/9029d4502623bd7390578c2c811a62516ef4a826/ --keypair ./wallets/deployer.json --upgrade-authority ./wallets/deployer.json --program-id ./wallets/v2/onlybags.json target/deploy/onlybags.so
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

## v2

> We deploy a new contract rather than upgrading. The reason is that the are currently tokens using the old curves so we don't what to upgrade because they will start using the new curves which are incompatible with the old ones.

Same as above but do not deploy a new Staking program. Just the Onlybags program.
