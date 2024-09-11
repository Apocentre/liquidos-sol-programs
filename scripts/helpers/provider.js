import * as anchor from "@coral-xyz/anchor";

const getClusterUrl = () => {
  switch(process.env.ENV) {
    case "dev":
      return "http://localhost:8899"
    case "devnet":
      return "https://smart-convincing-gadget.solana-devnet.quiknode.pro/d014cd6f46918d8dca9642ca6905c92a63096884/"
    case "mainnet":
      return "https://purple-ultra-silence.solana-mainnet.quiknode.pro/cdaa81c6c9635d407bdbad87774d95750b7818b2"
  }
}

export const provider = anchor.AnchorProvider.local(
  getClusterUrl(),
  {preflightCommitment: "confirmed"}
)

anchor.setProvider(provider);
