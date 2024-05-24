import * as anchor from "@coral-xyz/anchor";
import * as accounts from "./helpers/accounts.js";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "./helpers/provider.js";
import {createAndSendV0Tx} from "./helpers/tx.js";
import config from "./config.json" assert { type: "json" };
import tokenCreatorKey from "../wallets/test/tokenCreator.json" assert { type: "json" };

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {SystemProgram, PublicKey, Keypair} = anchor.web3


const main = async () => {
  const state = new PublicKey(config.state);
  const token = Keypair.generate();
  const tokenCreator = Keypair.fromSecretKey(Buffer.from(tokenCreatorKey))
  const program = anchor.workspace.Hodlhub;
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey)
  const bondingCurve = accounts.bondingCurve(state, token.publicKey, program.programId)[0];

  const ix = await program.methods
  .createToken(
    "TOKEN_NAME",
    "$TOKEN_SYMBOL",
    "TOKEN_URI"
  )
  .accounts({
    state,
    token: token.publicKey,
    tokenCreator: tokenCreator.publicKey,
    curveAta: await web3.getAssociatedTokenAddress(token.publicKey, bondingCurve, true, spl.TOKEN_2022_PROGRAM_ID),
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
    [tokenCreator, token]
  );

  console.log("State: ", state.publicKey.toBase58());
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
