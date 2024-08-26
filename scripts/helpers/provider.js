import * as anchor from "@coral-xyz/anchor";

const getClusterUrl = () => {
  switch(process.env.ENV) {
    case "dev":
      return "http://localhost:8899"
    case "devnet":
      return "https://blue-little-patina.solana-devnet.quiknode.pro/038172a7b63d22443ce416822a3cc5ed55d9dc8c/"
    case "mainnet":
      return "https://purple-ultra-silence.solana-mainnet.quiknode.pro/cdaa81c6c9635d407bdbad87774d95750b7818b2"
  }
}

export const provider = anchor.AnchorProvider.local(
  getClusterUrl(),
  {preflightCommitment: "confirmed"}
)

anchor.setProvider(provider);
