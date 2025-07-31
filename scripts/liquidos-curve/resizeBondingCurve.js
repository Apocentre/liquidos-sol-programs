import * as anchor from "@coral-xyz/anchor";
import Web3Pkg from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import {createAndSendV0Tx} from "../helpers/tx.js";

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {SystemProgram, PublicKey} = anchor.web3

const main = async () => {
  const state = new PublicKey("G5M4aCmU4KFRqppB2hJAwvKFJyUioZB1WaFeZoV6C3cz");
  const bondingCurve = new PublicKey("2AZKPN9xYu84D8jT93w7LWFpno5MpKFzPYnM7hniFkhQ");
  const program = anchor.workspace.LiquidosCurve;
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey);
  const newSize = new BN(250);

  const ix = await program.methods
  .resizeBondingCurve(newSize)
  .accounts({
    state,
    bondingCurve,
    payer: deployer.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .instruction();

  const priorityFeeIx = web3.setComputeUnitPrice(1000000);
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
