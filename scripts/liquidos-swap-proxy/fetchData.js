import * as anchor from "@coral-xyz/anchor";
import Web3Pkg from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import * as accounts from "../helpers/accounts.js";
import * as constants from "../helpers/constants.js";
import raydiumIDL from "./raydium_idl_devnet.json" with { type: "json" };

const Web3 = Web3Pkg.default;
const {PublicKey} = anchor.web3

export const fetchPoolState = async () => {
  const deployer = provider.wallet.payer;
  const web3 = Web3(deployer.publicKey);
  const token = new PublicKey("GQvnQnVVNdYsX1puzL5MAgoWtg7Bs4Toh5PQsNbyYwif")

  const raydiumProgram = constants.raydiumProgramDevnet;
  const ammConfig = constants.raydiumAmmConfigDevnet;
  const wsol = constants.wsol;
  const [token0, token1] = token.toBuffer() < wsol.toBuffer() ? [token, wsol] : [wsol, token];

  const raydiumCpmmProgram = await web3.createProgram(raydiumIDL);
  const poolState = accounts.raydiumPoolState(ammConfig, token0, token1, raydiumProgram)[0];
  return await raydiumCpmmProgram.account.poolState.fetch(poolState);
}

const main = async () => {    
  const poolStateData = await fetchPoolState();
  console.log(">>>>>>>>>>>", poolStateData);
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
