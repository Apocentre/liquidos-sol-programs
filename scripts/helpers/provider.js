import * as anchor from "@coral-xyz/anchor";

const getClusterUrl = () => {
  switch(process.env.ENV) {
    case "dev":
      return "http://localhost:8899"
    case "devnet":
      return "https://api.devnet.solana.com"
    case "mainnet":
      return "https://intensive-alien-wish.solana-mainnet.quiknode.pro/77ca2cba4139e49efe0c0a891d16b2cfa31735bc/"
  }
}

export const provider = anchor.AnchorProvider.local(
  getClusterUrl(),
  {preflightCommitment: "confirmed"}
)

anchor.setProvider(provider);
