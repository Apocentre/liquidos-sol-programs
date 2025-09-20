import * as anchor from "@coral-xyz/anchor";
import Web3Pkg from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";
import * as accounts from "../helpers/accounts.js";
import * as constants from "../helpers/constants.js";
import raydiumIDL from "./raydium_idl_devnet.json" with { type: "json" };

const Web3 = Web3Pkg.default;
const {PublicKey} = anchor.web3


const main = async () => {    
  const raydium = new RaydiumHelper();
  await raydium.create(provider.connection, "devnet");
  const poolId = new PublicKey("");
  const {poolInfo} = this.getPoolInfo(poolId);

  console.log(">>>>>>>>>>>", poolInfo);
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
