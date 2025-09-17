import * as anchor from "@coral-xyz/anchor";
import * as accounts from "../helpers/accounts.js";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import {createAndSendV0Tx} from "../helpers/tx.js";
import config from "../config.v3.json" with { type: "json" };
import sellerKey from "../../wallets/deployer_devnet.json" with { type: "json" };

const Web3 = Web3Pkg.default;
const {BN} = anchor.default;
const {SystemProgram, PublicKey, Keypair} = anchor.web3

const main = async () => {
  const deployer = provider.wallet.payer;
  const program = anchor.workspace.LiquidosCurve;
  const web3 = Web3(deployer.publicKey);
  const seller = Keypair.fromSecretKey(Buffer.from(sellerKey))
  const state = new PublicKey(config.liquidosCurveState);
  const token = new PublicKey("6MzojUhVkMjXG1TcwGfNd3pkX7fQay6aZwr9cSG9Hhos")
  const bondingCurve = accounts.bondingCurve(state, token, program.programId)[0];
  const amount = new BN(100000000);
  const minAmountOut = new BN(0); // no slippage
  const sellerAta = await web3.getAssociatedTokenAddress(token, seller.publicKey, true, spl.TOKEN_2022_PROGRAM_ID);
  const eventAuthority = accounts.eventAuthority(program.programId)[0];
  const treasuries = config.treasuries.map(({acc}) => ({
    pubkey: new PublicKey(acc),
    isSigner: false,
    isWritable: true,
  }));


  const sellIx = await program.methods
  .sell(amount, minAmountOut)
  .accounts({
    seller: seller.publicKey,
    state,
    bondingCurve,
    token,
    sellerAta,
    associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    token2022: spl.TOKEN_2022_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
    eventAuthority,
    program: program.programId,
  })
  .remainingAccounts(treasuries)
  .instruction();

  const priorityFeeIx = web3.setComputeUnitPrice(80000);
  await createAndSendV0Tx(
    provider,
    [priorityFeeIx, sellIx],
    seller.publicKey,
    [seller],
    [],
  );
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
