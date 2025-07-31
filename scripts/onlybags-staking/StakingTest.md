# Steps

1. Deploy the liquidos and the liquidos-staking programs:

```bash
solana program deploy --with-compute-unit-price 1000000  --max-sign-attempts 1000 --url https://smart-convincing-gadget.solana-devnet.quiknode.pro/d014cd6f46918d8dca9642ca6905c92a63096884/ --keypair ./wallets/deployer_devnet.json --upgrade-authority ./wallets/deployer_devnet.json --program-id ./wallets/test/onlybags_v2.json target/deploy/liquidos_curve.so

solana program deploy --with-compute-unit-price 1000000  --max-sign-attempts 1000 --url https://smart-convincing-gadget.solana-devnet.quiknode.pro/d014cd6f46918d8dca9642ca6905c92a63096884/ --keypair ./wallets/deployer_devnet.json --upgrade-authority ./wallets/deployer_devnet.json --program-id ./wallets/test/staking.json target/deploy/liquidos_staking.so
```

2. initialize Liquidos Curve program

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/liquidos-curve/initialize.js
```

Set `liquidosCurveState` in config.json

3. initialize staking program

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/liquidos-staking/initialize.js
```

Set `stakingState` in config.json

4. update liquidos curve state

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/liquidos/updateState.js
```

5. create bonding curve

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/liquidos/createToken.js
```

Set `stakingToken` and `rewardToken` in config.json to the value of the newly created token

6. buy from bonding curve

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/liquidos-curve/buy.js
```

7. stake

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/liquidos-staking/deposit.js
```

8. check user_info

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/liquidos-staking/fetchData.js
```


9. read pending rewards

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/liquidos-staking/readPendingReward.js
```

10. check user_info

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/liquidos-staking/fetchData.js
```

11. unstake

```bash
ENV=devnet ANCHOR_WALLET=./wallets/deployer_devnet.json node ./scripts/liquidos-staking/withdraw.js
```
