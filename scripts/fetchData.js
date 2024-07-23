import * as anchor from "@coral-xyz/anchor";
import * as accounts from "./helpers/accounts.js";
import * as provider from "./helpers/provider.js";
import config from "./config.json" assert { type: "json" };

const {PublicKey} = anchor.web3;

const getTokenAccount = (state, program) => {
  const tokenName = "Great Shibaa v100";
  const tokenSymbol= "GreatSHiba";
  return accounts.curveToken(state, tokenName, tokenSymbol, program.programId)[0];
}

const main = async () => {
  const program = anchor.workspace.Onlybags;
  const state = new PublicKey(config.state);
  const stateData = await program.account.state.fetch(state);
  const bondingCurve = new PublicKey("8eQzosSqy9PQLm5afzXV5URBuef9KoL9i4FPv9dycqh5"); //accounts.bondingCurve(state, getTokenAccount(state, program), program.programId)[0];
  const bondingCurveData = await program.account.bondingCurve.fetch(bondingCurve);

  // console.log("state: ", stateData)
  console.log("bondingCurve: ", {
    totalSupply: bondingCurveData.totalSupply.toString(),
    circulatingSupply: bondingCurveData.circulatingSupply.toString(),
    reserveTokenBalance: bondingCurveData.reserveTokenBalance.toString(),
    price: bondingCurveData.price.toString(),
    closed: bondingCurveData.closed,
    creatorFee: bondingCurveData.creatorFee.toString(),
    protocolFee: bondingCurveData.protocolFee.toString(),
    creatorFee: bondingCurveData.creatorFee.toString(),
    tradeFeeBps: bondingCurveData.tradeFeeBps.toString(),
  })
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
