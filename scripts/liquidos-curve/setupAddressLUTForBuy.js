import * as anchor from "@coral-xyz/anchor";
import {provider} from "../helpers/provider.js";
import * as accounts from "../helpers/accounts.js";
import {spl} from "@apocentre/solana-web3";
import * as constants from "../helpers/constants.js";
import {createAddressLUT, addAddressesToAddressLUT} from "../helpers/tx.js";
import config from "../config.v2.1.json" with { type: "json" };

const {SystemProgram, PublicKey, SYSVAR_RENT_PUBKEY} = anchor.web3

const main = async () => {
  const addressLUT = await createAddressLUT(provider);
  const liquidosCurveProgram = anchor.workspace.LiquidosCurve;
  const liqProgram = anchor.workspace.Liq;
  const ammConfig = constants.raydiumAmmConfigDevnet;
  const eventAuthority = accounts.eventAuthority(liquidosCurveProgram.programId)[0];
  const liqEventAuthority = accounts.eventAuthority(liqProgram.programId)[0];
  const raydiumProgram = constants.raydiumProgramDevnet;
  const raydiumAuthority = accounts.raydiumAuthority(raydiumProgram)[0];
  const liqState = new PublicKey(config.liqState);
  const liqBondingCurve = accounts.liqBondingCurve(liqState, liqProgram.programId)[0];

  const addresses = [
    new PublicKey(config.liquidosCurveState),
    liqState,
    liqBondingCurve,
    new PublicKey(config.treasury),
    raydiumProgram,
    raydiumAuthority,
    ammConfig,
    constants.wsol,
    spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    spl.TOKEN_PROGRAM_ID,
    spl.TOKEN_2022_PROGRAM_ID,
    SystemProgram.programId,
    SYSVAR_RENT_PUBKEY,
    liquidosCurveProgram.programId,
    liqProgram.programId,
    eventAuthority,
    liqEventAuthority,
  ];

  await addAddressesToAddressLUT(provider, addressLUT, addresses);

  console.log("AddressLUT: ", addressLUT.toBase58())
}

main()
.then(() => console.log("Success"))
.catch(error => console.log("Error: ", error))
