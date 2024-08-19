import * as anchor from "@coral-xyz/anchor";
import * as accounts from "../helpers/accounts.js";
import * as provider from "../helpers/provider.js";
import config from "../config.json" assert { type: "json" };

const {PublicKey} = anchor.web3;

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
  const stateData = await program.account.state.fetch(state);
  // const poolInfo = accounts.poolAuthority(state, getTokenAccount(), program.programId)[0];
  // const poolInfoData = await program.account.state.fetch(poolInfo);

  console.log("state: ", {
    owner: stateData.owner.toString(),
    onlybagsState: stateData.onlybagsState.toString(),
    treasury: stateData.treasury.toString(),
    stakingDuration: stateData.stakingDuration.toString(),
    stakingToken: stateData.stakingToken.toString(),
    poolCount: stateData.poolCount.toString(),
    protocolFee: stateData.protocolFee.toString(),
  });

  // console.log("poolInfoData: ", {
  //   accRewardPerShare: poolInfoData.accRewardPerShare.toString(),
  //   lastRewardTs: poolInfoData.lastRewardTs.toString(),
  //   endTs: poolInfoData.endTs.toString(),
  //   totalReward: poolInfoData.totalReward.toString(),
  //   totalStaked: poolInfoData.totalStaked.toString(),
  //   rewardPerSec: poolInfoData.rewardPerSec.toString(),
  //   rewardToken: poolInfoData.rewardToken.toString(),
  //   protocolFee: poolInfoData.protocolFee.toString(),
  // })
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
