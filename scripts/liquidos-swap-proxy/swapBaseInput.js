import * as anchor from "@coral-xyz/anchor";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import * as accounts from "../helpers/accounts.js";
import {createAndSendV0Tx} from "../helpers/tx.js";
import * as constants from "../helpers/constants.js";
import config from "../config.v3.json" with { type: "json" };
import RaydiumHelper from "./raydium.js"
import buyerKey from "../../wallets/deployer_devnet.json" with { type: "json" };

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {SystemProgram, PublicKey, Keypair} = anchor.web3

const main = async () => {
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey);
  await web3.init(provider.connection)
  const swapProxyProgram = anchor.workspace.LiquidosSwapProxy;
  const swapProxyState = new PublicKey(config.swapProxyState);
  const buyer = Keypair.fromSecretKey(Buffer.from(buyerKey))
  const token = new PublicKey("53NoJQN6i4aWgi2cdZSUrwzmo8KeABjXpEZRnmmB7Fos")

  const raydiumProgram = constants.raydiumProgramDevnet;
  const ammConfig = constants.raydiumAmmConfigDevnet;
  const wsol = constants.wsol;
  const [token0, token1] = token.toBuffer() < wsol.toBuffer() ? [token, wsol] : [wsol, token];
  const poolState = accounts.raydiumPoolState(ammConfig, token0, token1, raydiumProgram)[0];

  // buy TOKEN by selling WSOL. In the UI this is when the WSOL in above and token is below and we enter
  // the amount of WSOL we want to sell.
  // If we want to buy WSOL we need to switch moving WSOL down and TOKEN up and change 
  // inputTokenMint = token1 and outputTokenMint = token0;
  // Basically inputTokenMint is always the token that is up in the swap form
  const inputTokenMint = token0;
  const outputTokenMint = token1;
  const inputTokenProgram = inputTokenMint.equals(wsol)
    ? spl.TOKEN_PROGRAM_ID
    : spl.TOKEN_2022_PROGRAM_ID;
  const outputTokenProgram = outputTokenMint.equals(wsol)
    ? spl.TOKEN_PROGRAM_ID
    : spl.TOKEN_2022_PROGRAM_ID;

  const inputTokenAccount = inputTokenMint.equals(wsol)
    ? await web3.getAssociatedTokenAddress(inputTokenMint, buyer.publicKey)
    : await web3.getAssociatedTokenAddress(inputTokenMint, buyer.publicKey, true, spl.TOKEN_2022_PROGRAM_ID);

  const outputTokenAccount = outputTokenMint.equals(wsol)
    ? await web3.getAssociatedTokenAddress(outputTokenMint, buyer.publicKey)
    : await web3.getAssociatedTokenAddress(outputTokenMint, buyer.publicKey, true, spl.TOKEN_2022_PROGRAM_ID);

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

    const treasuryInputAta = inputTokenMint.equals(wsol)
      ? await web3.getAssociatedTokenAddress(inputTokenMint, treasury)
      : await web3.getAssociatedTokenAddress(inputTokenMint, treasury, true, spl.TOKEN_2022_PROGRAM_ID);

    remainingAccounts.push({
      pubkey: treasuryInputAta,
      isSigner: false,
      isWritable: true,
    });

    const treasuryOutputAta = outputTokenMint.equals(wsol) 
      ? await web3.getAssociatedTokenAddress(outputTokenMint, treasury)
      : await web3.getAssociatedTokenAddress(outputTokenMint, treasury, true, spl.TOKEN_2022_PROGRAM_ID);

    remainingAccounts.push({
      pubkey: new PublicKey(treasuryOutputAta),
      isSigner: false,
      isWritable: true,
    });
  }

  const wsolAmountToSwap = new BN(web3.toBase("1", 6));
  const tokenAmountToSwap = new BN(100_000000);
  const amountIn = inputTokenMint.equals(wsol) ? wsolAmountToSwap : tokenAmountToSwap;
  const raydium = new RaydiumHelper();
  await raydium.create(provider.connection, "devnet");
  const slippage = 0.01; // 1%
  const swapResult = await raydium.getSwapBaseInResult(poolState, inputTokenMint, amountIn, slippage)
  const minimumAmountOut = swapResult.outputAmount;

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

  const cbIx = web3.getComputationBudgetIx(250_000);
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
