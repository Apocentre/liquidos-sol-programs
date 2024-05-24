import * as anchor from "@coral-xyz/anchor";
import * as accounts from "./helpers/accounts.js";
import {provider} from "./helpers/provider.js";
import {createAndSendV0Tx} from "./helpers/tx.js";
import config from "./config.json" assert { type: "json" };
import buyerKey from "../wallets/test/buyer1.json" assert { type: "json" };

const {PublicKey, Keypair} = anchor.web3;

const getTokenAccount = (state, program) => {
  const tokenName = "TOKEN_1";
  const tokenSymbol= "SYMBOL_1";
  return accounts.curveToken(state, tokenName, tokenSymbol, program.programId)[0];
}

const main = async () => {
  const program = anchor.workspace.Hodlhub;
  const state = new PublicKey(config.state);
  const stateData = await program.account.state.fetch(state);
  const bondingCurve = accounts.bondingCurve(state, getTokenAccount(state, program), program.programId)[0];
  const bondingCurveData = await program.account.bondingCurve.fetch(bondingCurve);

  console.log("state: ", stateData)
  console.log("bondingCurve: ", {
    solTarget: bondingCurveData.solTarget.toString(),
    totalSupply: bondingCurveData.totalSupply.toString(),
    reserveTokenBalance: bondingCurveData.reserveTokenBalance.toString(),
    price: bondingCurveData.price.toString(),
    closed: bondingCurveData.closed,
  })
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
