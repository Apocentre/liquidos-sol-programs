import * as anchor from "@coral-xyz/anchor";
import {provider} from "../helpers/provider.js";
import {spl} from "@apocentre/solana-web3";
import * as constants from "../helpers/constants.js";
import {createAddressLUT, addAddressesToAddressLUT} from "../helpers/tx.js";
import config from "../config.json" assert { type: "json" };

const {SystemProgram, PublicKey} = anchor.web3

const main = async () => {
  const addressLUT = await createAddressLUT(provider);
  const ammConfig = constants.raydiumAmmConfigDevnet;
  const addresses = [
    new PublicKey(config.onlyBagsState),
    new PublicKey(config.treasury),
    ammConfig,
    constants.wsol,
    spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    spl.TOKEN_PROGRAM_ID,
    spl.TOKEN_2022_PROGRAM_ID,
    SystemProgram.programId
  ];

  await addAddressesToAddressLUT(provider, addressLUT, addresses);

  console.log("AddressLUT: ", addressLUT.toBase58())
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
