import * as anchor from "@coral-xyz/anchor";
import Web3Pkg, {spl} from "@apocentre/solana-web3";
import {provider} from "../helpers/provider.js";

const {PublicKey} = anchor.web3

const main = async () => {
  const mintAccount = await spl.getMint(
    provider.connection,
    new PublicKey("DTsTgfzjUYYepj4cQabJEbcC9FA4k1XwhEyx9pNpqsbz"),
    null,
    spl.TOKEN_2022_PROGRAM_ID
  );
  console.log("mintAccount: ", mintAccount);
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
