import * as anchor from "@coral-xyz/anchor";
import * as accounts from "../helpers/accounts.js";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import {createAndSendV0Tx} from "../helpers/tx.js";
import config from "../config.v2.1.json" with { type: "json" }

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {SystemProgram, PublicKey} = anchor.web3

const main = async () => {
  const program = anchor.workspace.Liq;
  const liquidosProgram = anchor.workspace.LiquidosCurve;
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey);
  const state = accounts.liqState();
  const bondingCurve = accounts.liqBondingCurve(state.publicKey, program.programId)[0];
  const eventAuthority = accounts.eventAuthority(program.programId)[0];
  const liqToken = accounts.liqToken(state.publicKey, "LIQ IOU", "LIQ", program.programId)[0];

  const ix = await program.methods
  .initialize(
    "LIQ IOU",
    "LIQ",
    "https://liquidos.fun/liq",
    liquidosProgram.programId,
    new PublicKey(config.liquidosCurveState),
    new BN(config.liqCreatorFeeBps),
  )
  .accounts({
    state: state.publicKey,
    bondingCurve,
    deployer: deployer.publicKey,
    liqToken,
    token2022: spl.TOKEN_2022_PROGRAM_ID,
    associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
    eventAuthority,
    program: program.programId,
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
