import * as anchor from "@coral-xyz/anchor";
import * as accounts from "../helpers/accounts.js";
import * as constants from "../helpers/constants.js";
import {provider} from "../helpers/provider.js";
import config from "../config.v2.1.json" with { type: "json" };

const {PublicKey} = anchor.web3;

const getTokenAccount = (state, program) => {
  return new PublicKey("")
}

const main = async () => {
  const program = anchor.workspace.LiquidosCurve;
  const state = new PublicKey("DALeEtZ2oebontPwba2hB3JD5KE4wzpwTaJW3ThxxAuv");
  const stateData = await program.account.state.fetch(state);
  const bondingCurve = accounts.bondingCurve(state, getTokenAccount(state, program), program.programId)[0];
  const bondingCurveData = await program.account.bondingCurve.fetch(bondingCurve);

  console.log("state: ", {
    owner: stateData.owner.toString(),
    treasury: stateData.treasury.toString(),
    protocolFee: stateData.protocolFee.toString(),
    tradeFeeBps: stateData.tradeFeeBps.toString(),
    creatorFee: stateData.creatorFee.toString(),
    totalTokenSupply: stateData.totalTokenSupply.toString(),
    stakingProgram: stateData.stakingProgram,
    stakingProgramState: stateData.stakingProgramState,
    stakingAllocation: stateData.stakingAllocation.toString(),
  });

  console.log("bondingCurve: ", {
    curveType: bondingCurveData.curveType,
    tokenCreator: bondingCurveData.tokenCreator.toString(),
    token: bondingCurveData.token.toString(),
    protocolFee: bondingCurveData.protocolFee.toString(),
    tradeFeeBps: bondingCurveData.tradeFeeBps.toString(),
    creatorFee: bondingCurveData.creatorFee.toString(),
    circulatingSupply: bondingCurveData.circulatingSupply.toString(),
    totalSupply: bondingCurveData.totalSupply.toString(),
    reserveTokenBalance: bondingCurveData.reserveTokenBalance.toString(),
    price: bondingCurveData.price.toString(),
    closed: bondingCurveData.closed,
    stakingAllocation: bondingCurveData.stakingAllocation.toString(),
  })
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
