import * as anchor from "@coral-xyz/anchor";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import * as accounts from "../helpers/accounts.js";
import config from "../config.json" assert { type: "json" };
import {createAndSendV0Tx} from "../helpers/tx.js";
import userKey from "../../wallets/deployer_devnet.json" assert { type: "json" };

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {SystemProgram, Keypair, PublicKey} = anchor.web3

const main = async () => {
  const state = new PublicKey(config.onlybagsLockerState)
  const user = Keypair.fromSecretKey(Buffer.from(userKey))
  const program = anchor.workspace.OnlybagsLocker;
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey);
  const lockAmount = new BN(web3.toBase("1000000", 6));
  const duration = new BN(10);
  const testTs = new BN(Number.MAX_SAFE_INTEGER);
  const escrow = accounts.escrow(state, program.programId)[0];
  const token = new PublicKey("DZCmJsjGWgydsDKHMfsAE3m6hUMFPBP5PWgHoryJHAKS");
  const tokenLock = accounts.tokenLock(state, token, program.programId)[0];
  const userLock = accounts.userLock(state, token, user.publicKey, program.programId)[0];
  const userAta = await web3.getAssociatedTokenAddress(token, user.publicKey, true, spl.TOKEN_2022_PROGRAM_ID);
  const escrowAta = await web3.getAssociatedTokenAddress(token, escrow, true, spl.TOKEN_2022_PROGRAM_ID);

  const ix = await program.methods
  .lock(lockAmount, duration, testTs)
  .accounts({
    state,
    token,
    tokenLock,
    userLock,
    userAta,
    escrow,
    escrowAta,
    user: user.publicKey,
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
    [deployer]
  );
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
