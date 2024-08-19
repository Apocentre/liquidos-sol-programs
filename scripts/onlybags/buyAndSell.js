import * as anchor from "@coral-xyz/anchor";
import * as accounts from "../helpers/accounts.js";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import {createAndSendV0Tx} from "../helpers/tx.js";
import * as constants from "../helpers/constants.js";
import config from "../config.json" assert { type: "json" };
import buyerKey1 from "../../wallets/deployer_devnet.json" assert { type: "json" };
import buyerKey2 from "../../wallets/test/buyer2.json" assert { type: "json" };

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {SystemProgram, PublicKey, Keypair, SYSVAR_RENT_PUBKEY} = anchor.web3

const buy = async (web3, buyer, state, token, bondingCurve, amount) => {
  const program = anchor.workspace.Onlybags;
  const minAmountOut = new BN(0); // no slippage
  const wsol = constants.wsol;
  const buyerAta = await web3.getAssociatedTokenAddress(token, buyer.publicKey, true, spl.TOKEN_2022_PROGRAM_ID);
  const buyerWsolAta = await web3.getAssociatedTokenAddress(wsol, buyer.publicKey);
  const ammConfig = constants.raydiumAmmConfigDevnet;
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
  
  const buy_ix = await program.methods
  .buy(amount, minAmountOut)
  .accounts({
    buyer: buyer.publicKey,
    state,
    treasury: new PublicKey(config.treasury),
    bondingCurve,
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

  const cbIx = web3.getComputationBudgetIx(750_000);
  await createAndSendV0Tx(
    provider,
    [cbIx, buy_ix, moveLiquidityIx],
    buyer.publicKey,
    [buyer],
    [],
  );
}

const sell = async (web3, seller, state, token, bondingCurve, amount) => {
  const program = anchor.workspace.OnlyBags;
  const minAmountOut = new BN(0); // no slippage
  const sellerAta = await web3.getAssociatedTokenAddress(token, seller.publicKey, true, spl.TOKEN_2022_PROGRAM_ID);

  const sell_ix = await program.methods
  .sell(amount, minAmountOut)
  .accounts({
    seller: seller.publicKey,
    state,
    treasury: new PublicKey(config.treasury),
    bondingCurve,
    token,
    sellerAta,
    associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    token2022: spl.TOKEN_2022_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
  })
  .instruction();

  await createAndSendV0Tx(
    provider,
    [sell_ix],
    seller.publicKey,
    [seller],
    [],
  );
}

const main = async () => {
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey);
  const program = anchor.workspace.Onlybags;
  const tokenName = "TOKEN_HUB_4";
  const tokenSymbol= "SYMBOL_HUB_4";
  const buyer1 = Keypair.fromSecretKey(Buffer.from(buyerKey1))
  const buyer2 = Keypair.fromSecretKey(Buffer.from(buyerKey2))
  const state = new PublicKey(config.onlyBagsState);
  const token = accounts.curveToken(state, tokenName, tokenSymbol, program.programId)[0];
  const bondingCurve = accounts.bondingCurve(state, token, program.programId)[0];
  
  await buy(web3, buyer1, state, token, bondingCurve, new BN(web3.toBase("5", 8))); // receives 21167548551095
  await buy(web3, buyer2, state, token, bondingCurve, new BN(web3.toBase("3", 8))); // receives 12016908098828
  await sell(web3, buyer1, state, token, bondingCurve, new BN(21167548551095))
  await sell(web3, buyer2, state, token, bondingCurve, new BN(12016908098828))

  await buy(web3, buyer1, state, token, bondingCurve, new BN(web3.toBase("8", 8)));
  await buy(web3, buyer2, state, token, bondingCurve, new BN(web3.toBase("10", 8))); // only .2 will be received
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
