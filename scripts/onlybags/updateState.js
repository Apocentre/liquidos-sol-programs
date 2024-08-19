import * as anchor from "@coral-xyz/anchor";
import * as accounts from "../helpers/accounts.js";
import Web3Pkg from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import {createAndSendV0Tx} from "../helpers/tx.js";
import config from "../config.json" assert { type: "json" };

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {PublicKey} = anchor.web3

const main = async () => {
  const state = accounts.state();
  const program = anchor.workspace.Onlybags;
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey);

  const ix = await program.methods
  .update_state(
    new PublicKey(config.stakingProgram),
    new PublicKey(config.stakingProgramState),
    new BN(config.protocolFee),
    new BN(config.tradeFeeBps),
    new BN(config.creatorFee),
    new BN(config.totalTokenSupply),
    new BN(config.staking_allocation_bps),
  )
  .accounts({
    state: state.publicKey,
    owner: deployer.publicKey,
  })
  .instruction();

  const priorityFeeIx = web3.setComputeUnitPrice(20000);
  await createAndSendV0Tx(
    provider,
    [priorityFeeIx, ix],
    deployer.publicKey,
    [deployer, state]
  );

}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
