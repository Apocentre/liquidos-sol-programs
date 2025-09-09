import * as anchor from "@coral-xyz/anchor";
import {provider} from "./provider.js";
import * as accounts from "./accounts.js";

const {PublicKey, sendAndConfirmTransaction, Transaction, SystemProgram} = anchor.web3;

import {
  ExtensionType,
  getMintLen,
  createInitializeMintInstruction,
  createInitializeMetadataPointerInstruction,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";


export const createToken = async (state, tokenCreator, tokenName, tokenSymbol, isTaxToken) => {
  const program = anchor.workspace.LiquidosCurve;
  // Define the extensions to be used by the mint
  const extensions = isTaxToken 
  ? [
      ExtensionType.MetadataPointer,
      ExtensionType.TransferFeeConfig,
    ]
  : [
      ExtensionType.MetadataPointer,
    ];

  // Calculate the length of the mint
  const mintLen = getMintLen(extensions);
  const mintLamports = await provider.connection.getMinimumBalanceForRentExemption(mintLen);
  const response = await fetch(`http://localhost:4000/tokens/vanity-addresses`, {
    method: 'POST',
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({
      token_name: tokenName,
      token_symbol: tokenSymbol,
      base: tokenCreator.toString(),
    })
  });
  const {seed, token_addr} = await response.json();
  const mint = new PublicKey(token_addr);
  const bondingCurve = accounts.bondingCurve(state, mint, program.programId)[0];
  const mintAuthority = bondingCurve;
  const decimals = 6;
  const ixs = [
    SystemProgram.createAccountWithSeed({
      basePubkey: tokenCreator,
      fromPubkey: tokenCreator,
      space: mintLen,
      lamports: mintLamports,
      newAccountPubkey: mint,
      programId: TOKEN_2022_PROGRAM_ID,
      seed,
    }),
    createInitializeMetadataPointerInstruction(mint, bondingCurve, mint, TOKEN_2022_PROGRAM_ID),
    createInitializeMintInstruction(mint, decimals, mintAuthority, null, TOKEN_2022_PROGRAM_ID),
  ]
  return [mint, ixs]
}
