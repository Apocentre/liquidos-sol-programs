import * as anchor from "@coral-xyz/anchor";
import * as accounts from "../helpers/accounts.js";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import {createAndSendV0Tx} from "../helpers/tx.js";
import config from "../config.json" assert { type: "json" };
import tokenCreatorKey from "../../wallets/deployer_devnet.json" assert { type: "json" };

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {SystemProgram, PublicKey, Keypair, SYSVAR_RENT_PUBKEY} = anchor.web3

const main = async () => {
  const tokenName = "TOKEN_TAX_HUB_2";
  const tokenSymbol= "SYMBOL_TAX_HUB_2";
  const state = new PublicKey(config.onlyBagsState);
  const tokenCreator = Keypair.fromSecretKey(Buffer.from(tokenCreatorKey))
  const program = anchor.workspace.Onlybags;
  const stakingProgram = anchor.workspace.OnlybagsStaking;
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey)
  const token = accounts.curveToken(state, tokenName, tokenSymbol, program.programId)[0];
  const bondingCurve = accounts.bondingCurve(state, token, program.programId)[0];
  const stakingState = new PublicKey(config.stakingState);
  const poolAuthority = accounts.poolAuthority(stakingState, stakingProgram.programId)[0];
  const poolInfo = accounts.poolInfo(stakingState, token, stakingProgram.programId)[0];
  const rewardTokenVaultAta = await web3.getAssociatedTokenAddress(token, poolAuthority, true, spl.TOKEN_2022_PROGRAM_ID);
  const stakingToken = token;
  const stakingTokenVaultAta = await web3.getAssociatedTokenAddress(stakingToken, poolAuthority, true, spl.TOKEN_2022_PROGRAM_ID);

  const ix = await program.methods
  .createTaxToken(
    tokenName,
    tokenSymbol,
    "http://onlybags.fun",
    new BN(200), // 2% transfer fee
    new BN(web3.toBase("20000000", 6)), // max fee that can be charged is 2% of the total supply i.e. 20M
  )
  .accounts({
    state,
    token,
    tokenCreator: tokenCreator.publicKey,
    curveAta: await web3.getAssociatedTokenAddress(token, bondingCurve, true, spl.TOKEN_2022_PROGRAM_ID),
    bondingCurve,
    associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    token2022: spl.TOKEN_2022_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
    rent: SYSVAR_RENT_PUBKEY,
  })
  .instruction();

  const createStakingPoolIx = await program.methods
  .createStakingPool()
  .accounts({
    payer: tokenCreator.publicKey,
    state,
    bondingCurve,
    token,
    stakingState,
    poolInfo,
    poolAuthority,
    stakingToken,
    stakingTokenVaultAta,
    rewardTokenVaultAta,
    stakingProgram: stakingProgram.programId,
    associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    tokenProgram: spl.TOKEN_PROGRAM_ID,
    token2022: spl.TOKEN_2022_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
  })
  .instruction();

  const priorityFeeIx = web3.setComputeUnitPrice(20000);
  await createAndSendV0Tx(
    provider,
    [priorityFeeIx, ix, createStakingPoolIx],
    tokenCreator.publicKey,
    [tokenCreator]
  );

  console.log("Token: ", token);
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
