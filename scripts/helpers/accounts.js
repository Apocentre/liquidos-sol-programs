import * as anchor from "@coral-xyz/anchor";

const {PublicKey, Keypair} = anchor.web3;
const utf8 = anchor.utils.bytes.utf8;

export const state = () => Keypair.generate()

export const bondingCurve = (state, token, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("bonding_curve"), state.toBuffer(), token.toBuffer()],
  programId
)

export const curveToken = (state, tokenName, tokenSymbol, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("hodlhub_token"), state.toBuffer(), utf8.encode(`${tokenName}-${tokenSymbol}`)],
  programId
)
