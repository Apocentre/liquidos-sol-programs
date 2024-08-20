import * as anchor from "@coral-xyz/anchor";
import * as accounts from "../helpers/accounts.js";
import {provider} from "../helpers/provider.js";
import config from "../config.json" assert { type: "json" };
import userKey from "../../wallets/deployer_devnet.json" assert { type: "json" };

const {Keypair, PublicKey} = anchor.web3

const main = async () => {
  const stakingState = new PublicKey(config.stakingState);
  const stakingProgram = anchor.workspace.OnlybagsStaking;
  const deployer = provider.wallet.payer;
  const user = Keypair.fromSecretKey(Buffer.from(userKey))

  const rewardToken = new PublicKey(config.rewardToken);
  const poolInfo = accounts.poolInfo(stakingState, rewardToken, stakingProgram.programId)[0];
  const stakingToken = new PublicKey(config.stakingToken)
  const userInfo = accounts.userInfo(stakingState, user.publicKey, rewardToken, stakingProgram.programId);


  const rewards = await stakingProgram.methods
  .readPendingReward(
    user.publicKey,
    stakingState,
    rewardToken,
    stakingToken,
  )
  .accounts({poolInfo, userInfo})
  .view();

  console.log("Pending rewards: ", rewards.toString());
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
