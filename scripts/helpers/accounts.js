import * as anchor from "@coral-xyz/anchor";

const {PublicKey, Keypair} = anchor.web3;
const utf8 = anchor.utils.bytes.utf8;

export const state = () => Keypair.generate()

export const bondingCurve = (state, token, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("bonding_curve"), state.toBuffer(), token.toBuffer()],
  programId
)

export const raydiumAuthority = (programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("vault_and_lp_mint_auth_seed")],
  programId
)

export const raydiumPoolState = (ammConfig, token0, token1, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("pool"), ammConfig.toBuffer(), token0.toBuffer(), token1.toBuffer()],
  programId
)

export const raydiumLpMint = (poolState, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("pool_lp_mint"), poolState.toBuffer()],
  programId
)

export const raydiumTokenVault = (poolState, token, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("pool_vault"), poolState.toBuffer(), token.toBuffer()],
  programId
)

export const raydiumObservationState = (poolState, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("observation"), poolState.toBuffer()],
  programId
)

export const poolAuthority = (state, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("pool_authority"), state.toBuffer()],
  programId
)

export const poolInfo = (state, rewardToken, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("staking_pool"), state.toBuffer(), rewardToken.toBuffer()],
  programId
)

export const userInfo = (state, user, rewardToken, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("user_info"), user.toBuffer(), state.toBuffer(), rewardToken.toBuffer()],
  programId
)

export const escrow = (state, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("escrow"), state.toBuffer()],
  programId
)

export const tokenLock = (state, token, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("token_lock"), state.toBuffer(), token.toBuffer()],
  programId
)

export const userLock = (state, token, user, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("user_lock"), state.toBuffer(), token.toBuffer(), user.toBuffer()],
  programId
)

export const eventAuthority = (programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("__event_authority")],
  programId,
)

export const liqState = () => Keypair.generate()

export const liqBondingCurve = (state, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("liq_bonding_curve"), state.toBuffer()],
  programId
)

export const liqToken = (state, tokenName, tokenSymbol, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("liq_token"), state.toBuffer(), utf8.encode(`${tokenName}-${tokenSymbol}`)],
  programId
)

export const buy_state = (token, buyer, programId) => PublicKey.findProgramAddressSync(
  [utf8.encode("buy_state"), token.toBuffer(), buyer.toBuffer()],
  programId
)
