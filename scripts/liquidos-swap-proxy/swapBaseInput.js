import * as anchor from "@coral-xyz/anchor";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import * as accounts from "../helpers/accounts.js";
import {createAndSendV0Tx} from "../helpers/tx.js";
import * as constants from "../helpers/constants.js";
import config from "../config.v3.json" with { type: "json" };
import buyerKey from "../../wallets/deployer_devnet.json" with { type: "json" };

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {SystemProgram, PublicKey, Keypair} = anchor.web3

const main = async () => {
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey);
  const swapProxyProgram = anchor.workspace.LiquidosSwapProxy;
  const swapProxyState = new PublicKey(config.swapProxyState);
  const buyer = Keypair.fromSecretKey(Buffer.from(buyerKey))
  const token = new PublicKey("GQvnQnVVNdYsX1puzL5MAgoWtg7Bs4Toh5PQsNbyYwif")

  const raydiumProgram = constants.raydiumProgramDevnet;
  const ammConfig = constants.raydiumAmmConfigDevnet;
  const wsol = constants.wsol;
  const [token0, token1] = token.toBuffer() < wsol.toBuffer() ? [token, wsol] : [wsol, token];
  const poolState = accounts.raydiumPoolState(ammConfig, token0, token1, raydiumProgram)[0];

  // Sell TOKEN for SOL. Swap the following values if you want a reverse. Note! use TOKEN_2022_PROGRAM_ID
  // where needed
  const inputTokenMint = token;
  const outputTokenMint = wsol;
  const inputTokenProgram = spl.TOKEN_2022_PROGRAM_ID;
  const outputTokenProgram = spl.TOKEN_PROGRAM_ID;
  const inputTokenAccount = await web3.getAssociatedTokenAddress(inputTokenMint, buyer.publicKey, true, spl.TOKEN_2022_PROGRAM_ID);
  const outputTokenAccount = await web3.getAssociatedTokenAddress(outputTokenMint, buyer.publicKey);
  const inputVault = accounts.raydiumTokenVault(poolState, inputTokenMint, raydiumProgram)[0];
  const outputVault = accounts.raydiumTokenVault(poolState, outputTokenMint, raydiumProgram)[0];
  const eventAuthority = accounts.eventAuthority(swapProxyProgram.programId)[0];

  const remainingAccounts = [];
  for(let t of config.treasuries) {
    const treasury = new PublicKey(t.acc);
    remainingAccounts.push({
      pubkey: treasury,
      isSigner: false,
      isWritable: true,
    });

    const treasuryInputAta = await web3.getAssociatedTokenAddress(inputTokenMint, treasury, true, spl.TOKEN_2022_PROGRAM_ID);
    remainingAccounts.push({
      pubkey: treasuryInputAta,
      isSigner: false,
      isWritable: true,
    });

    const treasuryOutputAta = await web3.getAssociatedTokenAddress(outputTokenMint, treasury);
    remainingAccounts.push({
      pubkey: new PublicKey(treasuryOutputAta),
      isSigner: false,
      isWritable: true,
    });
  }

  const amountIn = new BN(web3.toBase("1000000", 6));
  const minimumAmountOut = new BN(web3.toBase("0", 9));

  const swapBaseInputIx = await swapProxyProgram.methods
  .swapBaseInput(amountIn, minimumAmountOut)
  .accounts({
    payer: buyer.publicKey,
    state: swapProxyState,
    raydiumAuthority: accounts.raydiumAuthority(raydiumProgram)[0],
    ammConfig,
    poolState,
    inputTokenAccount,
    outputTokenAccount,
    inputVault,
    outputVault,
    inputTokenProgram,
    outputTokenProgram,
    inputTokenMint,
    outputTokenMint,
    observationState: accounts.raydiumObservationState(poolState, raydiumProgram)[0],
    cpSwapProgram: raydiumProgram,
    associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    tokenProgram: spl.TOKEN_PROGRAM_ID,
    token2022: spl.TOKEN_2022_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
    eventAuthority,
    program: swapProxyProgram.programId,
  })
  .remainingAccounts(remainingAccounts)
  .instruction();

  const cbIx = web3.getComputationBudgetIx(1_000_000);
  const priorityFeeIx = web3.setComputeUnitPrice(80000);
  await createAndSendV0Tx(
    provider,
    [cbIx, priorityFeeIx, swapBaseInputIx],
    buyer.publicKey,
    [buyer],
    [],
  );
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
