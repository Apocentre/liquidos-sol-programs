import * as anchor from "@coral-xyz/anchor";
import * as accounts from "./helpers/accounts.js";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "./helpers/provider.js";
import config from "./config.json" assert { type: "json" };
import account from "../wallets/test/buyer1.json" assert { type: "json" };

const {PublicKey, Keypair} = anchor.web3;

const getTokenAccount = (state, program) => {
  const tokenName = "TOKEN_HUB_3";
  const tokenSymbol= "SYMBOL_HUB_3";
  return accounts.curveToken(state, tokenName, tokenSymbol, program.programId)[0];
}

const main = async () => {
  const program = anchor.workspace.Hodlhub;
  // const state = new PublicKey(config.state);
  // const stateData = await program.account.state.fetch(state);
  // const bondingCurve = accounts.bondingCurve(state, getTokenAccount(state, program), program.programId)[0];
  // const bondingCurveData = await program.account.bondingCurve.fetch(bondingCurve);

  // console.log("state: ", stateData)
  // console.log("bondingCurve: ", {
  //   solTarget: bondingCurveData.solTarget.toString(),
  //   solTarget: bondingCurveData.solTarget.toString(),
  //   totalSupply: bondingCurveData.totalSupply.toString(),
  //   reserveTokenBalance: bondingCurveData.reserveTokenBalance.toString(),
  //   price: bondingCurveData.price.toString(),
  //   closed: bondingCurveData.closed,
  //   protocolFeeBps: bondingCurveData.protocolFeeBps.toString(),
  //   tradeFeeBps: bondingCurveData.tradeFeeBps.toString(),
  // })

  const Web3 = Web3Pkg.default;
  const web3 = Web3(account.publicKey);
  await web3.init(provider.connection)
  const userAta = new PublicKey("735kPodgkYGbmLwkfLBkMWJqdM7s8Yagukrxkpniwx4X")
  console.log(await web3.getTokenAccountBalance(userAta))
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
