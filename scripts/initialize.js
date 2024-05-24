import * as anchor from "@coral-xyz/anchor";
import * as accounts from "./helpers/accounts.js";
import {provider} from "./helpers/provider.js";
import {createAndSendV0Tx} from "./helpers/tx.js";
import config from "./config.json" assert { type: "json" };

const {BN} = anchor.default;
const {SystemProgram, PublicKey} = anchor.web3


const main = async () => {
  const state = accounts.state();
  const program = anchor.workspace.Hodlhub;
  const deployer = provider.wallet.payer;

  const ix = await program.methods
  .initialize(
    new PublicKey(config.treasury),
    new BN(config.solTarget),
    new BN(config.protocolFeeBps),
    new BN(config.tradeFeeBps),
  )
  .accounts({
    state: state.publicKey,
    owner: deployer.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .instruction();

  await createAndSendV0Tx(
    provider,
    [ix],
    deployer.publicKey,
    [deployer, state]
  );

  console.log("State: ", state.publicKey.toBase58());
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
