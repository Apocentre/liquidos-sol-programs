import * as anchor from "@coral-xyz/anchor";
import * as accounts from "../helpers/accounts.js";
import * as provider from "../helpers/provider.js";
import config from "../config.json" assert { type: "json" };
import userKey from "../../wallets/deployer_devnet.json" assert { type: "json" };

const {Keypair, PublicKey} = anchor.web3;

const getTokenAccount = () => {
  const program = anchor.workspace.Onlybags;
  const state = new PublicKey(config.onlyBagsState);
  const tokenName = "T_CURVE_2";
  const tokenSymbol= "S_CURVE_2";
  return accounts.curveToken(state, tokenName, tokenSymbol, program.programId)[0];
}

const main = async () => {
  const program = anchor.workspace.OnlybagsStaking;
  const state = new PublicKey(config.stakingState);
  const user = Keypair.fromSecretKey(Buffer.from(userKey))
  const stateData = await program.account.state.fetch(state);
  const poolInfo = accounts.poolInfo(state, getTokenAccount(), program.programId)[0];
  const poolInfoData = await program.account.poolInfo.fetch(poolInfo);

  console.log("state: ", {
    owner: stateData.owner.toString(),
    onlybagsState: stateData.onlybagsState.toString(),
    treasury: stateData.treasury.toString(),
    stakingDuration: stateData.stakingDuration.toString(),
    stakingToken: stateData.stakingToken.toString(),
    poolCount: stateData.poolCount.toString(),
    protocolFee: stateData.protocolFee.toString(),
  });

  console.log("poolInfoData: ", {
    accRewardPerShare: poolInfoData.accRewardPerShare.toString(),
    lastRewardTs: poolInfoData.lastRewardTs.toString(),
    endTs: poolInfoData.endTs.toString(),
    totalReward: poolInfoData.totalReward.toString(),
    totalStaked: poolInfoData.totalStaked.toString(),
    totalClaimed: poolInfoData.totalClaimed.toString(),
    rewardPerSec: poolInfoData.rewardPerSec.toString(),
    rewardToken: poolInfoData.rewardToken.toString(),
    protocolFee: poolInfoData.protocolFee.toString(),
  })

  const userInfo = accounts.userInfo(state, user.publicKey, poolInfoData.rewardToken, program.programId)[0];
  const userInfoData = await program.account.userInfo.fetch(userInfo);

  console.log("userInfo: ", {
    stakedAmount: userInfoData.stakedAmount.toString(),
    rewardDebt: userInfoData.rewardDebt.toString(),
    totalClaimed: userInfoData.totalClaimed.toString(),
    initialized: userInfoData.initialized.toString(),
  })
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
