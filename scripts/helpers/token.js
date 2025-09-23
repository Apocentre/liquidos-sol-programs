import * as anchor from "@coral-xyz/anchor";
import {provider} from "./provider.js";
import * as accounts from "./accounts.js";
import {
  ExtensionType,
  getMintLen,
  createInitializeMintInstruction,
  createInitializeMetadataPointerInstruction,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import nacl from "tweetnacl";
import naclUtil from "tweetnacl-util";

const {PublicKey, SystemProgram} = anchor.web3;

const login = async (tokenCreator) => {
  const ts = Date.now();
  const message = naclUtil.decodeUTF8(`WIF Auth:${ts}`);
  const sig = Buffer.from(
    nacl.sign.detached(message, tokenCreator.secretKey)
  ).toString("hex");

  const response = await fetch(`http://localhost:4000/accounts`, {
    method: 'POST',
    headers: {
      "X-Chain": "solana",
      "X-Platform": "wif",
      "X-Auth": `${ts}:${tokenCreator.publicKey.toString()}:${sig}`,
    }
  });

  const {jwt} = await response.json();

  return jwt;
}

export const createToken = async (state, tokenCreator) => {
  const jwt = await login(tokenCreator);
  const program = anchor.workspace.LiquidosCurve;
  // Define the extensions to be used by the mint
  const extensions = [
    ExtensionType.MetadataPointer,
  ];

  // Calculate the length of the mint
  const mintLen = getMintLen(extensions);
  const mintLamports = await provider.connection.getMinimumBalanceForRentExemption(mintLen);
  const response = await fetch(`http://localhost:4000/tokens/vanity-addresses?platform=wif`, {
    method: 'POST',
    headers: {
      "Content-Type": "application/json",
      "Authorization": `Bearer ${jwt}`
    }
  });
  const {seed, token_addr} = await response.json();
  const mint = new PublicKey(token_addr);
  const bondingCurve = accounts.bondingCurve(state, mint, program.programId)[0];
  const mintAuthority = bondingCurve;
  const decimals = 6;
  const ixs = [
    SystemProgram.createAccountWithSeed({
      basePubkey: tokenCreator.publicKey,
      fromPubkey: tokenCreator.publicKey,
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

export const createTaxToken = async (tokenCreator) => {
  const jwt = await login(tokenCreator);
  const response = await fetch(`http://localhost:4000/tokens/vanity-addresses`, {
    method: 'POST',
    headers: {
      "Content-Type": "application/json",
      "Authorization": `Bearer ${jwt}`
    }
  });

  const {seed, token_addr} = await response.json();
  const mint = new PublicKey(token_addr);

  return [seed, mint]
}
