> For devnet use the `wallets/deployer_devnet.json` wallet

## v1

1. Deploy the liquidos-curve and liquidos-staking programs:

```bash
solana program deploy --with-compute-unit-price 100000  --max-sign-attempts 10000 --url https://patient-tiniest-shadow.solana-mainnet.quiknode.pro/9029d4502623bd7390578c2c811a62516ef4a826/ --keypair ./wallets/deployer.json --upgrade-authority ./wallets/deployer.json --program-id ./wallets/staking.json target/deploy/liquidos_staking.so
```

```bash
solana program deploy --with-compute-unit-price 100000  --max-sign-attempts 10000 --url https://patient-tiniest-shadow.solana-mainnet.quiknode.pro/9029d4502623bd7390578c2c811a62516ef4a826/ --keypair ./wallets/deployer.json --upgrade-authority ./wallets/deployer.json --program-id ./wallets/v2/liquidos_curve.json target/deploy/liquidos_curve.so
```

2. initialize liquidos curve program

```bash
ENV=mainnet ANCHOR_WALLET=./wallets/deployer.json node ./scripts/liquidos-curve/initialize.js
```

Set `liquidosCurveState` in config.json

3. initialize staking program

```bash
ENV=mainnet ANCHOR_WALLET=./wallets/deployer.json node ./scripts/liquidos-staking/initialize.js
```

Set `stakingState` in config.json

4. update Liquidos Curve State

```bash
ENV=mainnet ANCHOR_WALLET=./wallets/deployer.json node ./scripts/liquidos-curve/updateState.js
```

## v2

> We deploy a new contract rather than upgrading. The reason is that the are currently tokens using the old curves so we don't what to upgrade because they will start using the new curves which are incompatible with the old ones.

Same as above but do not deploy a new Staking program. Just the Liquidos Curve program.

## v2.1

We need to deploy the `Liq` Program and upgrade the `Liquidos Curve`

```bash
solana program deploy --with-compute-unit-price 100000  --max-sign-attempts 10000 --url https://patient-tiniest-shadow.solana-mainnet.quiknode.pro/9029d4502623bd7390578c2c811a62516ef4a826/ --keypair ./wallets/deployer.json --upgrade-authority ./wallets/deployer.json --program-id ./wallets/v2.1/liq.json target/deploy/liq.so
```

```bash
solana program deploy --with-compute-unit-price 100000  --max-sign-attempts 10000 --url https://patient-tiniest-shadow.solana-mainnet.quiknode.pro/9029d4502623bd7390578c2c811a62516ef4a826/ --keypair ./wallets/deployer.json --upgrade-authority ./wallets/deployer.json --program-id ./wallets/v2/liquidos_curve.json target/deploy/liquidos_curve.so
```

3. Initialize Liq Program

```bash
ENV=mainnet ANCHOR_WALLET=./wallets/deployer.json node ./scripts/liq/initialize.js
```

## v3

Same as #V1 above but use this program id `./wallets/v3/liquidos_curve.json`
