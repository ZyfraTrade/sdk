import { AnchorProvider, Program, web3 } from "@coral-xyz/anchor"
import { analyzeInsight } from "../agents/intel_agent"

async function main() {
  const provider = AnchorProvider.env()
  const program = new Program(
    require("../target/idl/zyfra.json"),
    provider
  )

  const insight = web3.Keypair.generate()
  const rawData = "SOL liquidity anomaly detected"

  const { hash, score } = analyzeInsight(rawData)

  await program.methods
    .submitInsight([...hash])
    .accounts({
      insight: insight.publicKey,
      submitter: provider.wallet.publicKey,
      systemProgram: web3.SystemProgram.programId,
    })
    .signers([insight])
    .rpc()

  console.log("Insight submitted with score:", score)
}

main()
