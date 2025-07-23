import * as anchor from "@coral-xyz/anchor";
import * as accounts from "../helpers/accounts.js";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import {createAndSendV0Tx} from "../helpers/tx.js";
import config from "../config.v2.json" with { type: "json" };
import userKey from "../../wallets/deployer_devnet.json" with { type: "json" };

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {SystemProgram, Keypair, PublicKey} = anchor.web3

const main = async () => {
  const onlybagsState = new PublicKey(config.onlyBagsState);
  const stakingState = new PublicKey(config.stakingState);
  const stakingProgram = anchor.workspace.OnlybagsStaking;
  const onlybagsProgram = anchor.workspace.Onlybags;
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey);
  const user = Keypair.fromSecretKey(Buffer.from(userKey))

  const rewardToken = new PublicKey(config.rewardToken);
  const treasury = new PublicKey(config.treasury);
  const treasuryAta = await web3.getAssociatedTokenAddress(rewardToken, treasury, true, spl.TOKEN_2022_PROGRAM_ID);
  const poolInfo = accounts.poolInfo(stakingState, rewardToken, stakingProgram.programId)[0];
  const poolAuthority = accounts.poolAuthority(stakingState, stakingProgram.programId)[0];
  const rewardTokenVaultAta = await web3.getAssociatedTokenAddress(rewardToken, poolAuthority, true, spl.TOKEN_2022_PROGRAM_ID);
  const stakingToken = new PublicKey(config.stakingToken)
  const stakingTokenVaultAta = await web3.getAssociatedTokenAddress(stakingToken, poolAuthority, true, spl.TOKEN_2022_PROGRAM_ID);
  const userInfo = accounts.userInfo(stakingState, user.publicKey, rewardToken, stakingProgram.programId)[0];
  const userStakingAta =  await web3.getAssociatedTokenAddress(stakingToken, user.publicKey, true, spl.TOKEN_2022_PROGRAM_ID);
  const userRewardAta =  await web3.getAssociatedTokenAddress(rewardToken, user.publicKey, true, spl.TOKEN_2022_PROGRAM_ID);
  const bondingCurve = accounts.bondingCurve(onlybagsState, rewardToken, onlybagsProgram.programId)[0];
  const withdrawAmount = new BN(web3.toBase("0", 6));
  const eventAuthority = accounts.eventAuthority(stakingProgram.programId)[0];

  const ix = await stakingProgram.methods
  .withdraw(withdrawAmount)
  .accounts({
    state: stakingState,
    poolInfo,
    rewardToken,
    poolAuthority,
    treasury,
    treasuryAta,
    rewardTokenVaultAta,
    stakingToken,
    stakingTokenVaultAta,
    userInfo,
    userStakingAta,
    userRewardAta,
    bondingCurve,
    user: user.publicKey,
    associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    token2022: spl.TOKEN_2022_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
    eventAuthority,
    program: stakingProgram.programId,
  })
  .instruction();

  const priorityFeeIx = web3.setComputeUnitPrice(20000);
  await createAndSendV0Tx(
    provider,
    [priorityFeeIx, ix],
    user.publicKey,
    [user]
  );
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
