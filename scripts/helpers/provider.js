import * as anchor from "@coral-xyz/anchor";

const getClusterUrl = () => {
  switch(process.env.ENV) {
    case "dev":
      return "http://localhost:8899"
    case "devnet":
      return "https://smart-convincing-gadget.solana-devnet.quiknode.pro/d014cd6f46918d8dca9642ca6905c92a63096884/"
    case "mainnet":
      return "https://patient-tiniest-shadow.solana-mainnet.quiknode.pro/9029d4502623bd7390578c2c811a62516ef4a826"
  }
}

export const provider = anchor.AnchorProvider.local(
  getClusterUrl(),
  {preflightCommitment: "confirmed"}
)

anchor.setProvider(provider);
