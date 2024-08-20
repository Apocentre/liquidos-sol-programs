import * as anchor from "@coral-xyz/anchor";
import * as accounts from "../helpers/accounts.js";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import {createAndSendV0Tx} from "../helpers/tx.js";
import * as constants from "../helpers/constants.js";
import config from "../config.json" assert { type: "json" };
import buyerKey from "../../wallets/deployer_devnet.json" assert { type: "json" };

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {SystemProgram, PublicKey, Keypair, SYSVAR_RENT_PUBKEY} = anchor.web3

const main = async () => {
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey);
  const program = anchor.workspace.Onlybags;
  const stakingProgram = anchor.workspace.OnlybagsStaking;
  const tokenName = "T_CURVE_2";
  const tokenSymbol= "S_CURVE_2";
  const amount = new BN(web3.toBase("1", 9));
  const minAmountOut = new BN(0); // no slippage
  const buyer = Keypair.fromSecretKey(Buffer.from(buyerKey))
  const state = new PublicKey(config.onlyBagsState);
  const stakingState = new PublicKey(config.stakingState);
  const tokenCreator = new PublicKey(config.tokenCreator);
  const token = accounts.curveToken(state, tokenName, tokenSymbol, program.programId)[0];
  const buyerAta = await web3.getAssociatedTokenAddress(token, buyer.publicKey, true, spl.TOKEN_2022_PROGRAM_ID);
  const bondingCurve = accounts.bondingCurve(state, token, program.programId)[0];
  const wsol = constants.wsol;
  const buyerWsolAta = await web3.getAssociatedTokenAddress(wsol, buyer.publicKey);
  const ammConfig = constants.raydiumAmmConfigDevnet;
  const poolAuthority = accounts.poolAuthority(stakingState, stakingProgram.programId)[0];
  const poolInfo = accounts.poolInfo(stakingState, token, stakingProgram.programId)[0];
  const rewardTokenVaultAta = await web3.getAssociatedTokenAddress(token, poolAuthority, true, spl.TOKEN_2022_PROGRAM_ID);

  const buyIx = await program.methods
  .buy(amount, minAmountOut)
  .accounts({
    buyer: buyer.publicKey,
    state,
    treasury: new PublicKey(config.treasury),
    bondingCurve,
    tokenCreator,
    token,
    buyerAta,
    buyerWsolAta,
    wsolToken: wsol,
    ammConfig,
    associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    tokenProgram: spl.TOKEN_PROGRAM_ID,
    token2022: spl.TOKEN_2022_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
  })
  .instruction();

  const raydiumProgram = constants.raydiumProgramDevnet;
  const [token0, token1] = token.toBuffer() < wsol.toBuffer() ? [token, wsol] : [wsol, token];
  const poolState = accounts.raydiumPoolState(ammConfig, token0, token1, raydiumProgram)[0];
  const [creatorToken0, creatorToken1] = token.toBuffer() < wsol.toBuffer() ? [buyerAta, buyerWsolAta] : [buyerWsolAta, buyerAta];
  const lpMint = accounts.raydiumLpMint(poolState, raydiumProgram)[0];
  const creatorLpToken = await web3.getAssociatedTokenAddress(lpMint, buyer.publicKey);
  const [token0Vault, token1Vault] = token.toBuffer() < wsol.toBuffer()
  ? [accounts.raydiumTokenVault(poolState, token, raydiumProgram)[0], accounts.raydiumTokenVault(poolState, wsol, raydiumProgram)[0]] 
  : [accounts.raydiumTokenVault(poolState, wsol, raydiumProgram)[0], accounts.raydiumTokenVault(poolState, token, raydiumProgram)[0]];

  const createPoolFee = constants.raydiumCreatorPoolFeedDevnet;

  const moveLiquidityIx = await program.methods
  .moveLiquidity()
  .accounts({
    buyer: buyer.publicKey,
    state,
    bondingCurve,
    token,
    buyerAta,
    buyerWsolAta,
    ammConfig,
    raydiumAuthority: accounts.raydiumAuthority(raydiumProgram)[0],
    poolState,
    wsolToken: wsol,
    token0Mint: token0,
    token1Mint: token1,
    lpMint,
    creatorToken0,
    creatorToken1,
    creatorLpToken,
    token0Vault,
    token1Vault,
    createPoolFee,
    observationState: accounts.raydiumObservationState(poolState, raydiumProgram)[0],
    cpSwapProgram: raydiumProgram,
    associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    tokenProgram: spl.TOKEN_PROGRAM_ID,
    token2022: spl.TOKEN_2022_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
    rent: SYSVAR_RENT_PUBKEY,
  })
  .instruction();

  const createStakingPoolIx = await program.methods
  .createStakingPool()
  .accounts({
    buyer: buyer.publicKey,
    state,
    bondingCurve,
    token,
    stakingState,
    poolInfo,
    poolAuthority,
    rewardTokenVaultAta,
    stakingProgram: stakingProgram.programId,
    ammConfig,
    associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    tokenProgram: spl.TOKEN_PROGRAM_ID,
    token2022: spl.TOKEN_2022_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
  })
  .instruction();

  const cbIx = web3.getComputationBudgetIx(750_000);
  const priorityFeeIx = web3.setComputeUnitPrice(80000);
  await createAndSendV0Tx(
    provider,
    [cbIx, priorityFeeIx, buyIx, moveLiquidityIx, createStakingPoolIx],
    buyer.publicKey,
    [buyer],
    [],
  );
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
