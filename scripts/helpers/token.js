import * as anchor from "@coral-xyz/anchor";
import {provider} from "./provider.js";

const {PublicKey, sendAndConfirmTransaction} = anchor.web3;

import {
  ExtensionType,
  getMintLen,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";


export const createToken = async (state, tokenName, tokenSymbol) => {
  const payer = provider.wallet.payer;

  // Define the extensions to be used by the mint
  const extensions = [
    ExtensionType.MetadataPointer,
  ];

  // Calculate the length of the mint
  const seed = `onlybags_token:${state.toString()}:${tokenName}-${tokenSymbol}`;
  const mintLen = getMintLen(extensions);
  const mintLamports = await provider.connection.getMinimumBalanceForRentExemption(mintLen);
  const mintAcc = await PublicKey.createWithSeed(
    payer.publicKey,
    seed,
    TOKEN_2022_PROGRAM_ID,
  );

  const tx = new Transaction().add(
    SystemProgram.createAccountWithSeed({
      basePubkey: payer.publicKey,
      fromPubkey: payer.publicKey,
      space: mintLen,
      lamports: mintLamports,
      newAccountPubkey: mintAcc,
      programId: TOKEN_2022_PROGRAM_ID,
      seed,
    })
  );

  const newTokenTx = await sendAndConfirmTransaction(
    provider.connection,
    tx,
    [payer],
  );

  console.log("New Token Created:", newTokenTx);
}
