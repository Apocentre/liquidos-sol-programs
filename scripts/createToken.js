import * as anchor from "@coral-xyz/anchor";
import * as accounts from "./helpers/accounts.js";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "./helpers/provider.js";
import {createAndSendV0Tx} from "./helpers/tx.js";
import config from "./config.json" assert { type: "json" };
import tokenCreatorKey from "../wallets/test/tokenCreator.json" assert { type: "json" };

const Web3 = Web3Pkg.default;
const {SystemProgram, PublicKey, Keypair} = anchor.web3

const main = async () => {
  const tokenName = "TOKEN_4";
  const tokenSymbol= "SYMBOL_4";
  const state = new PublicKey(config.state);
  const tokenCreator = Keypair.fromSecretKey(Buffer.from(tokenCreatorKey))
  const program = anchor.workspace.Hodlhub;
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey)
  const token = accounts.curveToken(state, tokenName, tokenSymbol, program.programId)[0];
  const bondingCurve = accounts.bondingCurve(state, token, program.programId)[0];

  const ix = await program.methods
  .createToken(
    tokenName,
    tokenSymbol,
    "TOKEN_URI"
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
  })
  .instruction();

  await createAndSendV0Tx(
    provider,
    [ix],
    tokenCreator.publicKey,
    [tokenCreator]
  );

  console.log("Token: ", token);
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
