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
  const state = new PublicKey(config.onlyBagsState);
  const program = anchor.workspace.Onlybags;
  const stakingProgram = anchor.workspace.OnlybagsStaking;
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey);

  const ix = await program.methods
  .updateState(
    stakingProgram.programId,
    new PublicKey(config.stakingState),
    new BN(config.protocolFee),
    new BN(config.tradeFeeBps),
    new BN(config.creatorFee),
    new BN(config.totalTokenSupply),
    new BN(config.stakingAllocationBps),
  )
  .accounts({
    state,
    owner: deployer.publicKey,
  })
  .instruction();

  const priorityFeeIx = web3.setComputeUnitPrice(20000);
  await createAndSendV0Tx(
    provider,
    [priorityFeeIx, ix],
    deployer.publicKey,
    [deployer]
  );

}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
