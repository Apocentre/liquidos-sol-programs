import * as anchor from "@coral-xyz/anchor";

const getClusterUrl = () => {
  switch(process.env.ENV) {
    case "dev":
      return "http://localhost:8899"
    case "devnet":
      return "https://cool-capable-putty.solana-devnet.quiknode.pro/7c2a89805eb91da344d802a99e9fce6aa9932647"
    case "mainnet":
      return "https://purple-ultra-silence.solana-mainnet.quiknode.pro/cdaa81c6c9635d407bdbad87774d95750b7818b2"
  }
}

export const provider = anchor.AnchorProvider.local(
  getClusterUrl(),
  {preflightCommitment: "confirmed"}
)

anchor.setProvider(provider);
