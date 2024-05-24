import * as anchor from "@coral-xyz/anchor";

const getClusterUrl = () => {
  switch(process.env.ENV) {
    case "dev":
      return "http://localhost:8899"
    case "devnet":
      return "https://api.devnet.solana.com"
    case "mainnet":
      return ""
  }
}

export const provider = anchor.AnchorProvider.local(
  getClusterUrl(),
  {preflightCommitment: "confirmed"}
)

anchor.setProvider(provider);
