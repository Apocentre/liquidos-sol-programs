import * as anchor from "@coral-xyz/anchor";
import * as accounts from "./helpers/accounts.js";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "./helpers/provider.js";
import {createAndSendV0Tx} from "./helpers/tx.js";
import config from "./config.json" assert { type: "json" };
import tokenCreatorKey from "../wallets/deployer.json" assert { type: "json" };

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {SystemProgram, PublicKey, Keypair, SYSVAR_RENT_PUBKEY} = anchor.web3

const main = async () => {
  const tokenName = "TOKEN_HUB";
  const tokenSymbol= "SYMBOL_HUB";
  const state = new PublicKey(config.state);
  const tokenCreator = Keypair.fromSecretKey(Buffer.from(tokenCreatorKey))
  const program = anchor.workspace.Onlybags;
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey)
  const token = accounts.curveToken(state, tokenName, tokenSymbol, program.programId)[0];
  const bondingCurve = accounts.bondingCurve(state, token, program.programId)[0];

  const ix = await program.methods
  .createTaxToken(
    tokenName,
    tokenSymbol,
    "http://onlybags.fun",
    new BN(200), // 2% transfer fee
    new BN(200), // 2% max fee
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

  const priorityFeeIx = web3.setComputeUnitPrice(20000);
  await createAndSendV0Tx(
    provider,
    [priorityFeeIx, ix],
    tokenCreator.publicKey,
    [tokenCreator]
  );

  console.log("Token: ", token);
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
