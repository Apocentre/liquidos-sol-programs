import * as anchor from "@coral-xyz/anchor";
import * as accounts from "../helpers/accounts.js";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import {createAndSendV0Tx} from "../helpers/tx.js";
import config from "../config.json" assert { type: "json" };
import sellerKey from "../../wallets/deployer_devnet.json" assert { type: "json" };

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {SystemProgram, PublicKey, Keypair} = anchor.web3

const main = async () => {
  const deployer = provider.wallet.payer;
  const program = anchor.workspace.Onlybags;
  const web3 = Web3(deployer.publicKey);
  const tokenName = "T_16_CURVE_1";
  const tokenSymbol= "S_16_CURVE_1";
  const seller = Keypair.fromSecretKey(Buffer.from(sellerKey))
  const state = new PublicKey(config.onlyBagsState);
  const token = accounts.curveToken(state, tokenName, tokenSymbol, program.programId)[0];
  const bondingCurve = accounts.bondingCurve(state, token, program.programId)[0];
  const amount = new BN(14890926316);
  const minAmountOut = new BN(0); // no slippage
  const sellerAta = await web3.getAssociatedTokenAddress(token, seller.publicKey, true, spl.TOKEN_2022_PROGRAM_ID);

  const sellIx = await program.methods
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

  const priorityFeeIx = web3.setComputeUnitPrice(80000);
  await createAndSendV0Tx(
    provider,
    [priorityFeeIx, sellIx],
    seller.publicKey,
    [seller],
    [],
  );
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
