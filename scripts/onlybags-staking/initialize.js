import * as anchor from "@coral-xyz/anchor";
import * as accounts from "../helpers/accounts.js";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import {createAndSendV0Tx} from "../helpers/tx.js";
import config from "../config.json" assert { type: "json" };

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {SystemProgram, PublicKey} = anchor.web3

const main = async () => {
  const state = accounts.state();
  const program = anchor.workspace.OnlybagsStaking;
  const onlyBagsProgram = anchor.workspace.Onlybags;
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey);

  const poolAuthority = accounts.poolAuthority(state.publicKey, program.programId)[0];
  const stakingToken = new PublicKey(config.stakingToken)
  const stakingTokenVaultAta = await web3.getAssociatedTokenAddress(stakingToken, poolAuthority, true, spl.TOKEN_2022_PROGRAM_ID);

  const ix = await program.methods
  .initialize(
    onlyBagsProgram.programId,
    new PublicKey(config.onlyBagsState),
    new PublicKey(config.treasury),
    new BN(config.stakingDuration),
    new BN(config.stakingProtocolFee),
  )
  .accounts({
    state: state.publicKey,
    poolAuthority,
    stakingToken,
    stakingTokenVaultAta,
    owner: deployer.publicKey,
    associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    token2022: spl.TOKEN_2022_PROGRAM_ID,
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
