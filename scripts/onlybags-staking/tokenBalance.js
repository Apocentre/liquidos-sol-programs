import * as anchor from "@coral-xyz/anchor";
import * as accounts from "../helpers/accounts.js";
import * as provider from "../helpers/provider.js";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import config from "../config.json" assert { type: "json" };
import userKey from "../../wallets/deployer_devnet.json" assert { type: "json" };

const {Keypair, PublicKey} = anchor.web3;
const Web3 = Web3Pkg.default;

const main = async () => {
  const user = Keypair.fromSecretKey(Buffer.from(userKey))
  const web3 = Web3(user.publicKey);
  web3.connection = provider.provider.connection;

  const stakingState = new PublicKey(config.stakingState);
  const stakingProgram = anchor.workspace.OnlybagsStaking;
  const rewardToken = new PublicKey(config.rewardToken);
  const treasury = new PublicKey(config.treasury);
  const stakingToken = new PublicKey(config.stakingToken)
  const poolAuthority = accounts.poolAuthority(stakingState, stakingProgram.programId)[0];
  const rewardTokenVaultAta = await web3.getAssociatedTokenAddress(rewardToken, poolAuthority, true, spl.TOKEN_2022_PROGRAM_ID);
  const treasuryAta = await web3.getAssociatedTokenAddress(rewardToken, treasury, true, spl.TOKEN_2022_PROGRAM_ID);
  const stakingTokenVaultAta = await web3.getAssociatedTokenAddress(stakingToken, poolAuthority, true, spl.TOKEN_2022_PROGRAM_ID);
  const userStakingAta =  await web3.getAssociatedTokenAddress(stakingToken, user.publicKey, true, spl.TOKEN_2022_PROGRAM_ID);
  const userRewardAta =  await web3.getAssociatedTokenAddress(rewardToken, user.publicKey, true, spl.TOKEN_2022_PROGRAM_ID);
  
  console.log("userInfo: ", {
    rewardTokenVaultAta: (await web3.getTokenAccountBalance(rewardTokenVaultAta)),
    stakingTokenVaultAta: (await web3.getTokenAccountBalance(stakingTokenVaultAta)),
    userStakingAta: (await web3.getTokenAccountBalance(userStakingAta)),
    userRewardAta: (await web3.getTokenAccountBalance(userRewardAta)),
    treasuryAta: (await web3.getTokenAccountBalance(treasuryAta)),
  })
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
