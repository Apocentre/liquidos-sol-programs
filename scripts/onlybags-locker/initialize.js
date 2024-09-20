import * as anchor from "@coral-xyz/anchor";
import Web3Pkg from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import * as accounts from "../helpers/accounts.js";
import {createAndSendV0Tx} from "../helpers/tx.js";

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {SystemProgram} = anchor.web3

const main = async () => {
  const state = accounts.state();
  const program = anchor.workspace.OnlybagsLocker;
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey);
  const escrow = accounts.escrow(state.publicKey, program.programId)[0];

  const ix = await program.methods
  .initialize()
  .accounts({
    state: state.publicKey,
    owner: deployer.publicKey,
    escrow,
    systemProgram: SystemProgram.programId,
  })
  .instruction();

  const priorityFeeIx = web3.setComputeUnitPrice(20000);
  await createAndSendV0Tx(
    provider,
    [priorityFeeIx, ix],
    deployer.publicKey,
    [deployer, state]
  );

  console.log("State: ", state.publicKey.toBase58());
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
