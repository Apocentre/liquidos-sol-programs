import * as anchor from "@coral-xyz/anchor";
import * as accounts from "../helpers/accounts.js";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import {createAndSendV0Tx} from "../helpers/tx.js";
import * as constants from "../helpers/constants.js";
import config from "../config.json" assert { type: "json" };
import buyerKey from "../../wallets/test/buyer1.json" assert { type: "json" };

const Web3 = Web3Pkg.default;
const {SystemProgram, PublicKey, Keypair, SYSVAR_RENT_PUBKEY} = anchor.web3;

const main = async () => {
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey);
  const program = anchor.workspace.Onlybags;
  const tokenName = "TOKEN_3";
  const tokenSymbol= "SYMBOL_3";
  const buyer = Keypair.fromSecretKey(Buffer.from(buyerKey))
  const state = new PublicKey(config.onlyBagsState);
  const token = accounts.curveToken(state, tokenName, tokenSymbol, program.programId)[0];
  const bondingCurve = accounts.bondingCurve(state, token, program.programId)[0];
  
  const wsol = constants.wsol;
  const ammConfig = constants.raydiumAmmConfigDevnet;
  const raydiumProgram = constants.raydiumProgramDevnet;
  const [token0, token1] = token.toBuffer() < wsol.toBuffer() ? [token, wsol] : [wsol, token];
  const poolState = accounts.raydiumPoolState(ammConfig, token0, token1, raydiumProgram)[0];
  const buyerAta = await web3.getAssociatedTokenAddress(token, buyer.publicKey, true, spl.TOKEN_2022_PROGRAM_ID);
  const buyerWsolAta = await web3.getAssociatedTokenAddress(wsol, buyer.publicKey);
  const [creatorToken0, creatorToken1] = token.toBuffer() < wsol.toBuffer() ? [buyerAta, buyerWsolAta] : [buyerWsolAta, buyerAta];
  const lpMint = accounts.raydiumLpMint(poolState, raydiumProgram)[0];
  const creatorLpToken = await web3.getAssociatedTokenAddress(lpMint, buyer.publicKey);
  const [token0Vault, token1Vault] = token.toBuffer() < wsol.toBuffer()
  ? [accounts.raydiumTokenVault(poolState, token, raydiumProgram)[0], accounts.raydiumTokenVault(poolState, wsol, raydiumProgram)[0]] 
  : [accounts.raydiumTokenVault(poolState, wsol, raydiumProgram)[0], accounts.raydiumTokenVault(poolState, token, raydiumProgram)[0]];

  const createPoolFee = constants.raydiumCreatorPoolFeedDevnet;

  const ix = await program.methods
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
    associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    tokenProgram: spl.TOKEN_PROGRAM_ID,
    token2022: spl.TOKEN_2022_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
    rent: SYSVAR_RENT_PUBKEY,
  })
  .instruction();

  await createAndSendV0Tx(
    provider,
    [ix],
    buyer.publicKey,
    [buyer],
    [],
  );
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
